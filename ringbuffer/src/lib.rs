use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Ring {
    data: Vec<std::sync::atomic::AtomicU8>,
    size: usize,
    reader: AtomicUsize,
    writer: AtomicUsize,
    // the last place a push has occured, writes cannot go above this or the reader
    watermark: AtomicUsize,
}

/**
 * Ring is a circular buffer that can be shared between threads.
 * It can support multiple producers, but a single consumer.
 */
impl Ring {
    pub fn new(size: usize) -> Self {
        Self {
            data: (0..size)
                .map(|_| std::sync::atomic::AtomicU8::new(0))
                .collect(),
            size,
            reader: AtomicUsize::new(0),
            writer: AtomicUsize::new(0),
            watermark: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, value: u8) {
        let writer = self.writer.load(Ordering::Relaxed);
        let watermark = self.watermark.load(Ordering::Relaxed);

        // Only push if the watermark is not above the tail
        if writer < watermark {
            self.data[writer].store(value, Ordering::Relaxed);
            self.writer
                .store((writer + 1) % self.size, Ordering::Release);
        }
    }

    pub fn pop(&self) -> Option<u8> {
        let reader = self.reader.load(Ordering::Relaxed);
        if reader == self.writer.load(Ordering::Acquire) {
            return None; // Buffer is empty
        }
        let value = self.data[reader].load(Ordering::Relaxed);
        self.reader
            .store((reader + 1) % self.size, Ordering::Release);
        Some(value)
    }
}
