# Logging notes from gpt4:

You can use the `#[instrument]` attribute to automatically create and enter a span for a function. You can optionally include arguments in the span fields. Here's an example:

```rust
use tracing::{info, Instrument, instrument};
use tracing_futures::WithSubscriber;
use tokio;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("mycrate=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("set tracing default subscriber");
go
    handle_request(42).await;
}

#[instrument]
async fn get_user(user_id: u32) {
    info!(target: "get_user","Getting information for user with ID: {}", user_id);
    // The rest of your function...
}

#[instrument(fields(request_id))]
async fn handle_request(request_id: u64) {
    info!("Started handling request");

    get_user(123).await;
}
```

In this example, both `handle_request` and `get_user` create their own spans, which are entered for the duration of the async blocks. `get_user` will create a span with the target "get_user" and its `user_id` argument as a field. `handle_request` will create a span with an additional field `request_id`.

Remember to add a dependency for `tracing-futures` and to enable the `tokio` and `attributes` features for `tracing` in your `Cargo.toml`:

```toml
[dependencies]
tracing = { version = "0.1", features = ["attributes", "tokio"] }
tracing-futures = "0.2"
tracing-subscriber = "0.2"
tokio = { version = "0.3", features = ["full"] }
```
# learningRust
