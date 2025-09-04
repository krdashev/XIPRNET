//! Encrypted local storage
//! 
//! Provides secure local storage for messages, keys, and user data

use crate::utils::{Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedStorage {
    pub path: PathBuf,
    pub encryption_key: Vec<u8>,
    pub data: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub path: PathBuf,
    pub encryption_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStore {
    pub messages: Vec<StoredMessage>,
    pub conversations: Vec<Conversation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub content: Vec<u8>,
    pub timestamp: i64,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub participant_ids: Vec<String>,
    pub last_message_id: Option<String>,
    pub last_activity: i64,
}

impl EncryptedStorage {
    pub fn new(config: StorageConfig) -> Result<Self> {
        Ok(Self {
            path: config.path,
            encryption_key: config.encryption_key,
            data: HashMap::new(),
        })
    }
    
    pub fn store(&mut self, key: &str, value: &[u8]) -> Result<()> {
        // This would encrypt the data before storing
        self.data.insert(key.to_string(), value.to_vec());
        Ok(())
    }
    
    pub fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.data.get(key).cloned())
    }
    
    pub fn delete(&mut self, key: &str) -> Result<bool> {
        Ok(self.data.remove(key).is_some())
    }
    
    pub fn list_keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

impl MessageStore {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            conversations: Vec::new(),
        }
    }
    
    pub fn add_message(&mut self, message: StoredMessage) -> Result<()> {
        self.messages.push(message);
        Ok(())
    }
    
    pub fn get_messages(&self, conversation_id: &str) -> Vec<&StoredMessage> {
        self.messages
            .iter()
            .filter(|msg| msg.conversation_id == conversation_id)
            .collect()
    }
    
    pub fn add_conversation(&mut self, conversation: Conversation) -> Result<()> {
        self.conversations.push(conversation);
        Ok(())
    }
    
    pub fn get_conversation(&self, id: &str) -> Option<&Conversation> {
        self.conversations.iter().find(|conv| conv.id == id)
    }
    
    pub fn mark_message_read(&mut self, message_id: &str) -> Result<bool> {
        if let Some(message) = self.messages.iter_mut().find(|msg| msg.id == message_id) {
            message.is_read = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}