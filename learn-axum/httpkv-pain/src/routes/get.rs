use std::{fmt, str::FromStr};

use crate::{AppError, AppState};
use axum::{body::Bytes, extract::{Path, Query, State}};
// use axum_extra::extract::Query;
use serde::{de, Deserialize, Deserializer};
use validator::Validate;
use anyhow::anyhow;

#[derive(Deserialize, Debug, Validate)]
pub struct GetOrListParams {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    list: Option<String>,
}

// fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
// where
//     D: Deserializer<'de>,
//     T: FromStr,
//     T::Err: fmt::Display,
// {
//     let opt = Option::<String>::deserialize(de)?;
//     println!("OPT: {:?}", opt);
//     match opt.as_deref() {
//         None => Ok(None),
//         Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
//     }
// }

pub async fn get_root(
    State(state): State<AppState>,
    Query(params): Query<GetOrListParams>,
) -> Result<Bytes, AppError> {
    get_or_list_prefix(state, None, &params).await
}

pub async fn get_key(
    State(state): State<AppState>,
    Path(key_prefix): Path<String>,
    Query(params): Query<GetOrListParams>,
) -> Result<Bytes, AppError> {
    get_or_list_prefix(state, Some(key_prefix), &params).await
}

pub async fn get_or_list_prefix(
    state: AppState,
    key_prefix: Option<String>,
    params: &GetOrListParams,
) -> Result<Bytes, AppError> {
    println!("Got key: {:?} {:?}", key_prefix, params);
    params.validate()?;

    // Check if we are a list
    match &params.list {
        Some(list) if list.is_empty() => {
            println!("Is list");
            return listget(state, params, key_prefix).await
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


async fn get(state: AppState, params: &GetOrListParams, key: &String) -> Result<Bytes, AppError> {
    if let Some(val) = state.kv.get(key) {
        Ok(val.clone())
    } else {
        Err(anyhow!("not found").into())
    }
}

async fn listget(state: AppState, params: &GetOrListParams, prefix: Option<String>) -> Result<Bytes, AppError> {
    todo!()
}
