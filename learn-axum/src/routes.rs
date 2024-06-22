use crate::AppError;
use axum::{
    extract::Request,
    http::Method, response::{IntoResponse, Response},
};
use axum_extra::extract::Query;
use serde::Deserialize;
use validator::{Validate, ValidationError};

pub async fn handler() -> Result<(), AppError> {
    println!("got something");
    try_thing().await?;
    Ok(())
}

pub async fn try_thing() -> anyhow::Result<()> {
    anyhow::bail!("yeah")
}

#[derive(Deserialize, Debug, Validate)]
pub struct Params {
    #[serde(default)]
    hey: Vec<String>,
    #[serde(default)]
    // #[validate(length(min = 3, message = "If provided, should be at least 3 caracters"))]
    ho: Vec<String>, // Check if has single blank string for ?ho&...
}

pub async fn print_bytes(
    method: Method,
    Query(params): Query<Params>,
    // Query(params): Query<Vec<(String, String)>>,
    request: Request,
) -> Result<String, AppError> {
    // let method = request.method().clone();
    let bytes = axum::body::to_bytes(request.into_body(), usize::MAX)
        .await
        .unwrap();
    params.validate()?;
    println!("Got bytes ({}) {:?} {:?}", method, bytes, params);
    Ok(String::from_utf8(bytes.to_vec()).unwrap())
}
// pub async fn print_bytes(method: Method, body: Bytes) -> Result<(), AppError> {
//     let bytes: Vec<u8> = request.body().clone().into();
//     println!("Got bytes ({}) {:?}", request.method(), bytes);
//     Ok(())
// }
