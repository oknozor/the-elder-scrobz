use std::sync::{Arc, Mutex};

use elder_scrobz_db::PgPool;
use elder_scrobz_model::events::ScrobzEvent;
use tokio::sync::broadcast;

use crate::event_manager::EventManager;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub event_manager: Arc<Mutex<EventManager>>,
}

impl AppState {
    pub fn new(pool: PgPool, sse_sender: broadcast::Sender<ScrobzEvent>) -> Self {
        Self {
            db: pool,
            event_manager: Arc::new(Mutex::new(EventManager::new(sse_sender))),
        }
    }
}
