use learn_axum::start;

#[tokio::main]
async fn main() {
    start("0.0.0.0:8080").await
}
