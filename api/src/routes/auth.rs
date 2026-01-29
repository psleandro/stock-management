use crate::app::AppState;
use crate::handlers::auth::{signin, signup};
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}
