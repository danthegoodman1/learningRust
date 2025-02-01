use tokio::runtime::Handle;

// Your async function that you want to call
async fn my_async_function() -> String {
    // ... perform some async work ...
    "Hello from async!".to_string()
}

// A synchronous function running within a Tokio runtime.
fn my_sync_function() -> String {
    // If you are already inside a Tokio runtime, you should use block_in_place
    // to ensure that blocking the current thread won't interfere with other async tasks.
    tokio::task::block_in_place(|| {
        // Get a handle to the current runtime and block on the async function.
        Handle::current().block_on(my_async_function())
    })
}

#[tokio::main]
async fn main() {
    // Although main is async here, imagine that my_sync_function is called
    // from some synchronous code inside the runtime.
    let result = my_sync_function();
    println!("Result: {}", result);
}
