use crate::{AppError, AppState};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
};
// use axum_extra::extract::Query;
use anyhow::anyhow;
use serde::Deserialize;
use tokio::time::Instant;
use tracing::{debug, info};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct WriteParams {
    #[serde(default, alias = "ix")]
    if_exists: Option<String>,

    #[serde(default, alias = "nx")]
    not_exists: Option<String>,

    #[serde(default, alias = "v")]
    version: Option<i64>,
}

#[tracing::instrument(level = "debug", skip(state))]
pub async fn write_key(
    Path(key): Path<String>,
    State(state): State<AppState>,
    Query(params): Query<WriteParams>,
    body: Bytes,
) -> Result<String, AppError> {
    {
        let val = state.kv.read().await;
        let val = val.get(&key);
        match val {
            Some(_) => {
                if let Some(_) = params.not_exists {
                    if state.kv.read().await.contains_key(&key) {
                        return Err(AppError::CustomCode(
                            anyhow!("Key {} exists (nx)", key),
                            axum::http::StatusCode::CONFLICT,
                        ));
                    }
                }
                if let Some(version) = params.version {
                    todo!("check version")
                }
            }
            None => {
                if let Some(_) = params.if_exists {
                    // Check that it exists first
                    if !state.kv.read().await.contains_key(&key) {
                        return Err(AppError::CustomCode(
                            anyhow!("Key {} doesn't exist (ix)", key),
                            axum::http::StatusCode::CONFLICT,
                        ));
                    }
                }
            }
        };
    }

    // Write the value
    state.kv.write().await.insert(
        key,
        crate::Item {
            timestamp: Instant::now().elapsed().as_nanos(),
            data: body.into(),
        },
    );
    info!("wrote it");

    Ok("".to_string())
}
