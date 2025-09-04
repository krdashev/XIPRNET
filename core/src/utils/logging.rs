//! Secure logging implementation
//! 
//! Provides secure logging with sensitive data redaction

use tracing::{info, warn, error};

pub fn init_logging() {
    tracing_subscriber::fmt::init();
}

pub fn log_message_sent(message_id: &str, recipient_id: &str) {
    info!("Message sent: {} to {}", message_id, recipient_id);
}

pub fn log_message_received(message_id: &str, sender_id: &str) {
    info!("Message received: {} from {}", message_id, sender_id);
}

pub fn log_auth_success(user_id: &str) {
    info!("Authentication successful for user: {}", user_id);
}

pub fn log_auth_failure(user_id: &str, reason: &str) {
    warn!("Authentication failed for user: {} - {}", user_id, reason);
}

pub fn log_crypto_error(operation: &str, error: &str) {
    error!("Crypto error in {}: {}", operation, error);
}

pub fn log_storage_error(operation: &str, error: &str) {
    error!("Storage error in {}: {}", operation, error);
}

pub fn redact_sensitive_data(data: &str) -> String {
    // Simple redaction - in practice would be more sophisticated
    if data.len() > 8 {
        format!("{}***{}", &data[..4], &data[data.len()-4..])
    } else {
        "***".to_string()
    }
}
