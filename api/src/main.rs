use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

use crate::{app::build_app, infrastructure::db};

pub mod app;
pub mod errors;
pub mod extractors;
pub mod handlers;
pub mod infrastructure;
pub mod models;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = db::create_db_pool();

    let app = build_app(db_pool);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("Failed to parse PORT!");

    let socket_addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("App listening on {}", socket_addr);

    let listener = TcpListener::bind(socket_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
