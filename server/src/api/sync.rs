//! Synchronization API endpoints

use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
};
use serde::{Serialize};
use xipr_core::storage::sync::{SyncRequest, SyncResponse};

#[derive(Debug, Serialize)]
pub struct SyncApiResponse {
    pub success: bool,
    pub data: Option<SyncResponse>,
    pub error: Option<String>,
}

pub async fn sync_messages(
    Json(request): Json<SyncRequest>,
) -> Result<JsonResponse<SyncApiResponse>, StatusCode> {
    match xipr_core::storage::sync::SyncManager::sync_messages(&request) {
        Ok(sync_response) => Ok(JsonResponse(SyncApiResponse {
            success: true,
            data: Some(sync_response),
            error: None,
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}