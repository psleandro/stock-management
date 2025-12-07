use std::{env, net::SocketAddr};
use dotenvy::dotenv;
use tokio::net::TcpListener;

use crate::app::AppState;

pub mod app;
pub mod extractors;
pub mod infrastructure;
pub mod routes;
pub mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = AppState::new();

    let app = routes::app_routes().with_state(app_state);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("Failed to parse PORT!");

    let socket_addr = SocketAddr::from(([127,0,0,1], port));
    println!("App listening on {}", socket_addr);

    let listener = TcpListener::bind(socket_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

