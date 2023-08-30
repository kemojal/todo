// src/app_state.rs

use sqlx::PgPool;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub db_pool: Arc<Mutex<PgPool>>,
}
