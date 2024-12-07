// Use this to wait for destruction of something without the Drop being sync (but exit will be sync)

#[derive(Default)]
pub struct ImportantTasks(Arc<(AtomicU64, AtomicWaker)>);

impl ImportantTasks {
    pub fn handle(&self) -> Handle {
        Handle(self.0.clone())
    }
}

impl Future for ImportantTasks {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        self.0 .1.register(cx.waker());
        if self.0 .0.load(atomic::Ordering::Acquire) == 0 {
            return Poll::Ready(());
        }
        Poll::Pending
    }
}

#[derive(Clone)]
pub struct Handle(Arc<(AtomicU64, AtomicWaker)>);

impl Handle {
    pub fn spawn<F: 'static + Send + Future<Output = ()>>(&self, future: F) {
        struct Guard(Arc<(AtomicU64, AtomicWaker)>);
        impl Drop for Guard {
            fn drop(&mut self) {
                if self.0 .0.fetch_sub(1, atomic::Ordering::Release) == 1 {
                    self.0 .1.wake();
                }
            }
        }

        self.0 .0.fetch_add(1, atomic::Ordering::Relaxed);
        let guard = Guard(self.0.clone());
        tokio::spawn(async move {
            let _guard = guard;
            future.await;
        });
    }
}

use futures_util::task::AtomicWaker;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::task;
use std::task::Poll;
