use xipr_core::protocol::auth::{Session, User};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AuthService {
    users: Mutex<HashMap<String, User>>,
    sessions: Mutex<HashMap<String, Session>>,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            sessions: Mutex::new(HashMap::new()),
        }
    }
    
    pub fn create_user(&self, username: String, email: String) -> Result<User, String> {
        let user = User::new(username, email);
        let mut users = self.users.lock().unwrap();
        users.insert(user.id.clone(), user.clone());
        Ok(user)
    }
    
    pub fn get_user(&self, user_id: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(user_id).cloned()
    }
    
    pub fn create_session(&self, user_id: String, device_id: String) -> Session {
        let session = Session::new(user_id, device_id);
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.token.clone(), session.clone());
        session
    }
    
    pub fn validate_session(&self, token: &str) -> Option<Session> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(token) {
            if session.is_expired() {
                sessions.remove(token);
                None
            } else {
                Some(session.clone())
            }
        } else {
            None
        }
    }
    
    pub fn revoke_session(&self, token: &str) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(token).is_some()
    }
}