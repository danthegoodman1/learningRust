use std::{collections::BTreeMap, sync::Arc};

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use tokio::sync::RwLock;
use tracing::info;

mod routes;

#[derive(Clone, Debug)]
struct AppState {
    kv: Arc<RwLock<BTreeMap<String, Item>>>,
}

#[derive(Clone, Debug)]
struct Item {
    version: i64,
    data: Vec<u8>,
}

pub async fn start(addr: &str) {
    let state = AppState {
        kv: Arc::new(RwLock::new(BTreeMap::new())),
    };
    let app = axum::Router::new()
        .route("/", get(routes::get::get_root))
        .route(
            "/:key",
            get(routes::get::get_key).post(routes::post::write_key),
        )
        .with_state(state)
        .layer(DefaultBodyLimit::max(99_000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Starting on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// Make our own error that wraps `anyhow::Error`.
pub enum AppError {
    Anyhow(anyhow::Error),
    CustomCode(anyhow::Error, axum::http::StatusCode),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Anyhow(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", e),
            ),
            AppError::CustomCode(e, code) => (code, format!("{}", e)),
        }
        .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Anyhow(err.into())
    }
}
