use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::app::AppState;
use crate::handlers::auth::signup;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/signup", post(signup))
}