use axum::{Router, routing::post};

use crate::ServerState;
use crate::handlers::auth::{login, register};

pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
