use crate::{AppError, AppState};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
};
// use axum_extra::extract::Query;
use anyhow::anyhow;
use serde::Deserialize;
use validator::Validate;
use tracing::{info, debug};

#[derive(Deserialize, Debug, Validate)]
pub struct GetOrListParams {
    list: Option<String>,
}

#[tracing::instrument(level="debug", skip(state))]
pub async fn get_root(
    State(state): State<AppState>,
    Query(params): Query<GetOrListParams>,
) -> Result<Bytes, AppError> {
    get_or_list_prefix(state, None, &params).await
}

#[tracing::instrument(level="debug", skip(state))]
pub async fn get_key(
    State(state): State<AppState>,
    Path(key_prefix): Path<String>,
    Query(params): Query<GetOrListParams>,
) -> Result<Bytes, AppError> {
    get_or_list_prefix(state, Some(key_prefix), &params).await
}

#[tracing::instrument(level="debug", skip(state))]
pub async fn get_or_list_prefix(
    state: AppState,
    key_prefix: Option<String>,
    params: &GetOrListParams,
) -> Result<Bytes, AppError> {
    tracing::info!("Got key: {:?} {:?}", key_prefix, params);
    params.validate()?;

    // Check if we are a list
    match &params.list {
        Some(list) if list.is_empty() => {
            println!("Is list");
            return listget(state, params, key_prefix).await;
        }
        _ => {}
    }

    if let Some(key) = key_prefix {
        println!("Is get");
        get(state, params, &key).await
    } else {
        // Just a health check
        Ok("alive".into())
    }
}

#[tracing::instrument(level="debug", skip(state))]
async fn get(state: AppState, params: &GetOrListParams, key: &String) -> Result<Bytes, AppError> {
    if let Some(val) = state.kv.get(key) {
        Ok(val.clone())
    } else {
        Err(anyhow!("not found").into())
    }
}

#[tracing::instrument(level="debug", skip(state))]
async fn listget(
    state: AppState,
    params: &GetOrListParams,
    prefix: Option<String>,
) -> Result<Bytes, AppError> {
    todo!()
}
