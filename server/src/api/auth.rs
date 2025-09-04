//! Authentication API endpoints

use axum::{
    extract::Json,
    http::StatusCode,
    response::Json as JsonResponse,
};
use serde::{Deserialize, Serialize};
use xipr_core::protocol::auth::{AuthRequest, Session, User};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub device_id: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub user_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub session: Option<Session>,
    pub error: Option<String>,
}

pub async fn register(
    Json(payload): Json<RegisterRequest>,
) -> Result<JsonResponse<RegisterResponse>, StatusCode> {
    // This would integrate with OPAQUE registration
    let user = User::new(payload.username, payload.email);
    
    Ok(JsonResponse(RegisterResponse {
        success: true,
        user_id: Some(user.id),
        error: None,
    }))
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<JsonResponse<LoginResponse>, StatusCode> {
    let auth_request = AuthRequest {
        username: payload.username,
        password: payload.password,
        device_id: payload.device_id,
    };
    
    match xipr_core::protocol::auth::AuthManager::authenticate(&auth_request) {
        Ok(auth_response) => {
            if auth_response.success {
                Ok(JsonResponse(LoginResponse {
                    success: true,
                    session: auth_response.session,
                    error: None,
                }))
            } else {
                Ok(JsonResponse(LoginResponse {
                    success: false,
                    session: None,
                    error: auth_response.error,
                }))
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}