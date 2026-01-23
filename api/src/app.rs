use std::sync::Arc;

use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;

use crate::infrastructure::db::db;

#[derive(Clone)]
pub struct AppState {
  pub db_pool: Pool<Manager<PgConnection>>,
}

impl AppState {
    pub fn new() -> Arc<AppState> {
        let db_pool = db::create_db_pool();

        Arc::new(AppState { db_pool })
    }
}
