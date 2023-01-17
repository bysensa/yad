use std::sync::Arc;

use crate::db::{Database, GetDb};

#[derive(Clone)]
pub struct AppState {
    db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        AppState { db: Arc::new(db) }
    }
}

impl GetDb for AppState {
    fn get_db(&self) -> &Database {
        &self.db
    }
}
