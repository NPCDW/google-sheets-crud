use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub config: crate::config::config::Config,
    pub token: Arc<RwLock<Option<String>>>,
    pub token_exp: Arc<RwLock<Option<u64>>>,
}