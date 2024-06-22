use crate::{AppError, AppState};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use base64::prelude::*;
// use axum_extra::extract::Query;
use anyhow::anyhow;
use serde::Deserialize;
use tracing::{debug, info};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct GetOrListParams {
    list: Option<String>,

    // List params
    limit: Option<i64>,
    #[serde(default, alias = "nx")]
    with_vals: Option<String>,
    start: Option<i64>,
    end: Option<i64>
}

#[tracing::instrument(level = "debug", skip(state))]
pub async fn get_root(
    State(state): State<AppState>,
    Query(params): Query<GetOrListParams>,
) -> Result<Response, AppError> {
    get_or_list_prefix(state, None, &params).await
}

#[tracing::instrument(level = "debug", skip(state))]
pub async fn get_key(
    State(state): State<AppState>,
    Path(key_prefix): Path<String>,
    Query(params): Query<GetOrListParams>,
) -> Result<Response, AppError> {
    get_or_list_prefix(state, Some(key_prefix), &params).await
}

#[tracing::instrument(level = "debug", skip(state))]
pub async fn get_or_list_prefix(
    state: AppState,
    key_prefix: Option<String>,
    params: &GetOrListParams,
) -> Result<Response, AppError> {
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
        Ok("alive".into_response())
    }
}

#[tracing::instrument(level = "debug")]
async fn get(
    state: AppState,
    params: &GetOrListParams,
    key: &String,
) -> Result<Response, AppError> {
    let kv = state.kv.read().await;
    debug!("Map: {:?}", kv);
    if let Some(val) = kv.get(key) {
        let mut body = val.data.clone();
        if params.start.is_some() || params.end.is_some() {
            // We need to get a subslice of the body
            let start = params.start.or(Some(0)).unwrap() as usize;
            let end = params.end.or(Some(body.len() as i64)).unwrap() as usize;
            debug!("Getting subslice of value for key {} with start={} end={}", key, start, end);
            body = body[start..end].to_vec();
        }

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("version", HeaderValue::from(val.timestamp))
            .body(body.into())
            .expect("Failed to construct response"))
    } else {
        Err(anyhow!("not found").into())
    }
}

#[tracing::instrument(level = "debug", skip(state))]
async fn listget(
    state: AppState,
    params: &GetOrListParams,
    prefix: Option<String>,
) -> Result<Response, AppError> {
    let mut items: Vec<String> = Vec::new();
    let kv = state.kv.read().await;
    let with_vals = params.with_vals.is_some();

    // Build the list of items
    let range_start = prefix.or(Some(String::from(""))).unwrap();
    let take = params.limit.or(Some(100)).unwrap() as usize;
    debug!("Using range start '{}' take={} with_vals={}", range_start, take, with_vals);
    for (key, item) in kv
        .range(range_start..)
        .take(take)
    {
        if with_vals {
            items.push(format!(
                "{}:{}",
                key,
                BASE64_STANDARD.encode(item.data.clone())
            ));
            continue;
        }
        items.push(key.clone());
    }

    Ok(items.join("\n").into_response())
}
