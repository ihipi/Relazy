use crate::domain::models::user::User;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub token: Arc<Mutex<Option<String>>>,
    pub user: Arc<Mutex<Option<User>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
            user: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_token(&mut self, token: String) {
        let mut self_token = self.token.lock().unwrap();
        *self_token = Some(token);
    }
}
