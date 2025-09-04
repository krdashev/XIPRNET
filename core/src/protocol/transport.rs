//! Message transport and framing
//! 
//! Provides message serialization, routing, and delivery

use crate::utils::{Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub message_type: MessageType,
    pub content: Vec<u8>,
    pub timestamp: i64,
    pub sequence_number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Binary,
    Control,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFrame {
    pub header: MessageHeader,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub version: u8,
    pub message_type: MessageType,
    pub message_id: String,
    pub sender_id: String,
    pub recipient_id: String,
    pub timestamp: i64,
    pub sequence_number: u64,
    pub payload_length: u32,
}

impl Message {
    pub fn new(
        sender_id: String,
        recipient_id: String,
        message_type: MessageType,
        content: Vec<u8>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sender_id,
            recipient_id,
            message_type,
            content,
            timestamp: chrono::Utc::now().timestamp(),
            sequence_number: 0,
        }
    }
    
    pub fn to_frame(&self) -> Result<MessageFrame> {
        let header = MessageHeader {
            version: 1,
            message_type: self.message_type.clone(),
            message_id: self.id.clone(),
            sender_id: self.sender_id.clone(),
            recipient_id: self.recipient_id.clone(),
            timestamp: self.timestamp,
            sequence_number: self.sequence_number,
            payload_length: self.content.len() as u32,
        };
        
        let payload = serde_json::to_vec(&self.content)?;
        
        Ok(MessageFrame {
            header,
            payload,
        })
    }
    
    pub fn from_frame(frame: &MessageFrame) -> Result<Self> {
        let content: Vec<u8> = serde_json::from_slice(&frame.payload)?;
        
        Ok(Self {
            id: frame.header.message_id.clone(),
            sender_id: frame.header.sender_id.clone(),
            recipient_id: frame.header.recipient_id.clone(),
            message_type: frame.header.message_type.clone(),
            content,
            timestamp: frame.header.timestamp,
            sequence_number: frame.header.sequence_number,
        })
    }
}

pub struct MessageRouter;

impl MessageRouter {
    pub fn route_message(message: &Message) -> Result<String> {
        // Simple routing logic - in practice would be more sophisticated
        Ok(message.recipient_id.clone())
    }
    
    pub fn validate_message(message: &Message) -> Result<bool> {
        // Validate message integrity and format
        if message.id.is_empty() || message.sender_id.is_empty() || message.recipient_id.is_empty() {
            return Ok(false);
        }
        
        if message.timestamp <= 0 {
            return Ok(false);
        }
        
        Ok(true)
    }
}
