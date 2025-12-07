use std::env;

use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;

pub fn create_db_pool() -> Pool<Manager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    let pool = Pool::builder(manager)
        .build()
        .unwrap();

    pool
}
