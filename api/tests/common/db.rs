use std::env;

use deadpool_diesel::{Manager, Pool, Runtime};
use diesel::{PgConnection, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use tokio::sync::{Mutex, OnceCell};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static TEST_POOL: OnceCell<Pool<Manager<PgConnection>>> = OnceCell::const_new();
static TEST_DB_LOCK: OnceCell<Mutex<()>> = OnceCell::const_new();

pub async fn create_test_pool() -> &'static Pool<Manager<PgConnection>> {
    TEST_POOL
        .get_or_init(|| async {
            dotenv().ok();

            let database_url =
                env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");

            let manager = Manager::new(database_url, Runtime::Tokio1);

            let pool = Pool::builder(manager)
                .build()
                .expect("Failed to create test pool");

            run_migrations(&pool).await;
            clean_db(&pool).await;

            pool
        })
        .await
}

pub async fn lock_test_db() -> tokio::sync::MutexGuard<'static, ()> {
    TEST_DB_LOCK.get_or_init(|| async { Mutex::new(()) }).await.lock().await
}

pub async fn clean_db(pool: &Pool<Manager<PgConnection>>) {
    let conn = pool.get().await.unwrap();

    conn.interact(|conn| {
        diesel::sql_query(
            "TRUNCATE TABLE stock_movements, places, products, suppliers, workspaces, users RESTART IDENTITY CASCADE",
        )
        .execute(conn)
    })
    .await
    .unwrap()
    .unwrap();
}

async fn run_migrations(pool: &Pool<Manager<PgConnection>>) {
    pool.get()
        .await
        .expect("Error on get pool")
        .interact(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .map(|_| ())
                .expect("Error running pending migrations")
        })
        .await
        .expect("Error on interaction setup");
}
