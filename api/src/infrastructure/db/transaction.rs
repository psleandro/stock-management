use deadpool_diesel::{Manager, Pool};
use diesel::Connection;

use crate::errors::InfrastructureError;

pub struct TransactionRunner {
    pool: Pool<Manager<diesel::PgConnection>>,
}

impl TransactionRunner {
    pub fn new(pool: Pool<Manager<diesel::PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn run<F, T>(&self, f: F) -> Result<T, InfrastructureError>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
        T: Send + 'static,
    {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Connection(e.to_string()))?;

        let result = conn
            .interact(|conn| conn.transaction::<T, diesel::result::Error, _>(f))
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(result)
    }
}
