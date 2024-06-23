use crate::{AppError, AppState};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
// use axum_extra::extract::Query;
use anyhow::anyhow;
use serde::Deserialize;
use tracing::{debug, debug_span, event};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct GetOrListParams {
    list: Option<String>,

    // List params
    limit: Option<i64>,
    #[serde(default, alias = "vals")]
    with_vals: Option<String>,
    reverse: Option<String>,
    start: Option<i64>,
    end: Option<i64>,
}

pub async fn get_root(
    State(state): State<AppState>,
    Query(params): Query<GetOrListParams>,
) -> Result<Response, AppError> {
    get_or_list_prefix(state, None, &params).await
}

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
    params.validate()?;

    // Check if we are a list
    match &params.list {
        Some(list) if list.is_empty() => {
            return list_items(state, params, key_prefix).await;
        }
        _ => {}
    }

    if let Some(key) = key_prefix {
        get_item(state, params, &key).await
    } else {
        // Just a health check
        Ok("alive".into_response())
    }
}

#[tracing::instrument(level = "debug", skip(state))]
async fn get_item(
    state: AppState,
    params: &GetOrListParams,
    key: &String,
) -> Result<Response, AppError> {
    let kv = state.kv.read().await;
    if let Some(val) = kv.get(key) {
        let mut body = val.data.clone();
        if params.start.is_some() || params.end.is_some() {
            // We need to get a subslice of the body
            let start = params.start.or(Some(0)).unwrap() as usize;
            let end = params.end.or(Some(body.len() as i64)).unwrap() as usize;
            debug!(
                "Getting subslice of value for key {} with start={} end={}",
                key, start, end
            );
            body = body[start..end].to_vec();
        }

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("version", HeaderValue::from(val.version))
            .body(body.into())
            .expect("Failed to construct response"))
    } else {
        Err(AppError::CustomCode(anyhow!("not found"), StatusCode::NOT_FOUND))
    }
}

#[tracing::instrument(level = "debug", skip(state))]
async fn list_items(
    state: AppState,
    params: &GetOrListParams,
    prefix: Option<String>,
) -> Result<Response, AppError> {
    let mut items: Vec<u8> = Vec::new();
    let kv = state.kv.read().await;
    let with_vals = params.with_vals.is_some();

    // Build the list of items
    let range_start = prefix.clone().or(Some(String::from(""))).unwrap(); // "" is beginning of DB
    let range_end = match prefix {
        Some(p) => p.as_bytes().to_owned(),
        None => vec![0xFF], // 0xFF is end of DB
    };
    let range_end_str = String::from_utf8(range_end).unwrap();
    let reverse = params.reverse.is_some();
    let limit = params.limit.or(Some(100)).unwrap() as usize;
    debug!(
        start = range_start,
        end = range_end_str,
        limit = limit,
        with_vals = with_vals,
        reverse = reverse,
        "Listing items",
    );

    // Sometimes rust is a PITA and you just repeat yourself instead
    if !reverse {
        // Forward iterate
        for (key, item) in kv.range(range_start..).take(limit) {
            // Add the key
            if with_vals {
                items.extend(key.as_bytes());
                items.extend("\n".as_bytes());
                items.extend(item.data.clone());
                items.extend("\n".as_bytes());
                items.extend("\n".as_bytes());
            } else {
                items.extend(key.as_bytes());
                items.extend("\n".as_bytes());
            }
        }
    } else {
        // Reverse iterate

        // If we have a prefix, use it
        let rev_range = match range_start.as_str() {
            "" => {
                let last_item = kv.last_key_value();
                match last_item {
                    Some(item) => {
                        kv.range(..item.0.to_owned() + "~").rev() // temp for the map, just add something larger on the end, fdb client has just options for the range iterator https://docs.rs/foundationdb/latest/foundationdb/struct.RangeOption.html
                    }
                    None => kv.range(.."".to_string()).rev(),
                }
            }
            _ => kv.range(..range_start).rev(),
        };
        for (key, item) in rev_range.take(limit) {
            // Add the key
            if with_vals {
                items.extend(key.as_bytes());
                items.extend("\n".as_bytes());
                items.extend(item.data.clone());
                items.extend("\n".as_bytes());
                items.extend("\n".as_bytes());
            } else {
                items.extend(key.as_bytes());
                items.extend("\n".as_bytes());
            }
        }
    }

    let mut sep_len = 0;
    if items.len() > 0 {
        let sep = match with_vals {
            true => "\n\n",
            false => "\n",
        }
        .as_bytes();
        sep_len = sep.len();
    }

    Ok(items[0..items.len() - sep_len].to_vec().into_response())
}
