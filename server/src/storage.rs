//! Storage service for XIPRNET server

use xipr_core::storage::local::{MessageStore, StoredMessage, Conversation};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct StorageService {
    message_store: Mutex<MessageStore>,
    user_storage: Mutex<HashMap<String, Vec<u8>>>,
}

impl StorageService {
    pub fn new() -> Self {
        Self {
            message_store: Mutex::new(MessageStore::new()),
            user_storage: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn store_message(&self, message: StoredMessage) -> Result<(), String> {
        let mut store = self.message_store.lock().unwrap();
        store.add_message(message).map_err(|e| e.to_string())
    }
    
    pub fn get_messages(&self, conversation_id: &str) -> Vec<StoredMessage> {
        let store = self.message_store.lock().unwrap();
        store.get_messages(conversation_id).to_vec().into_iter().cloned().collect()
    }
    
    pub fn add_conversation(&self, conversation: Conversation) -> Result<(), String> {
        let mut store = self.message_store.lock().unwrap();
        store.add_conversation(conversation).map_err(|e| e.to_string())
    }
    
    pub fn get_conversation(&self, id: &str) -> Option<Conversation> {
        let store = self.message_store.lock().unwrap();
        store.get_conversation(id).cloned()
    }
    
    pub fn store_user_data(&self, user_id: &str, data: Vec<u8>) {
        let mut storage = self.user_storage.lock().unwrap();
        storage.insert(user_id.to_string(), data);
    }
    
    pub fn get_user_data(&self, user_id: &str) -> Option<Vec<u8>> {
        let storage = self.user_storage.lock().unwrap();
        storage.get(user_id).cloned()
    }
    
    pub fn delete_user_data(&self, user_id: &str) -> bool {
        let mut storage = self.user_storage.lock().unwrap();
        storage.remove(user_id).is_some()
    }
}