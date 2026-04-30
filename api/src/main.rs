use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use stock_management_api::{build_app, db};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool = db::create_db_pool();

    let app = build_app(db_pool);

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("Failed to parse PORT!");

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("App listening on {}", socket_addr);

    let listener = TcpListener::bind(socket_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
