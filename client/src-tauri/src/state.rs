use crate::models::{AppSettings, ConnectionStats, UserInfo};
use std::sync::Mutex;

pub struct AppState {
    pub user: Mutex<Option<UserInfo>>,
    pub token: Mutex<Option<String>>,
    pub connection_stats: Mutex<ConnectionStats>,
    pub settings: Mutex<AppSettings>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            user: Mutex::new(None),
            token: Mutex::new(None),
            connection_stats: Mutex::new(ConnectionStats::default()),
            settings: Mutex::new(AppSettings::default()),
        }
    }

    pub fn set_user(&self, user: UserInfo) {
        *self.user.lock().unwrap() = Some(user);
    }

    pub fn get_user(&self) -> Option<UserInfo> {
        self.user.lock().unwrap().clone()
    }

    pub fn set_token(&self, token: String) {
        *self.token.lock().unwrap() = Some(token);
    }

    pub fn get_token(&self) -> Option<String> {
        self.token.lock().unwrap().clone()
    }

    pub fn clear_auth(&self) {
        *self.user.lock().unwrap() = None;
        *self.token.lock().unwrap() = None;
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.lock().unwrap().is_some()
    }

    pub fn update_stats(&self, stats: ConnectionStats) {
        *self.connection_stats.lock().unwrap() = stats;
    }

    pub fn get_stats(&self) -> ConnectionStats {
        self.connection_stats.lock().unwrap().clone()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
