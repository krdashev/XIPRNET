//! Message API endpoints

use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::Json as JsonResponse,
};
use serde::{Deserialize, Serialize};
use xipr_core::protocol::transport::Message;

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub recipient_id: String,
    pub content: Vec<u8>,
    pub message_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub conversation_id: String,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct SendMessageResponse {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetMessagesResponse {
    pub messages: Vec<Message>,
    pub has_more: bool,
}

pub async fn send_message(
    Json(payload): Json<SendMessageRequest>,
) -> Result<JsonResponse<SendMessageResponse>, StatusCode> {
    // This would validate the request and store the message
    let message = Message::new(
        "sender_id".to_string(), // Would come from auth
        payload.recipient_id,
        xipr_core::protocol::transport::MessageType::Text,
        payload.content,
    );
    
    Ok(JsonResponse(SendMessageResponse {
        success: true,
        message_id: Some(message.id),
        error: None,
    }))
}

pub async fn get_messages(
    Query(_query): Query<GetMessagesQuery>,
) -> Result<JsonResponse<GetMessagesResponse>, StatusCode> {
    // This would fetch messages from storage
    let messages = vec![]; // Would be fetched from database
    
    Ok(JsonResponse(GetMessagesResponse {
        messages,
        has_more: false,
    }))
}