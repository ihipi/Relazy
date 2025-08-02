use std::sync::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub token: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_token(&mut self, token: String) {
        let mut self_token = self.token.lock().unwrap();
        *self_token = Some(token);
    }
}
