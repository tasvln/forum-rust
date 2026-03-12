use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::posts::{create_post, get_post, get_posts},
    states::server::ServerState,
};

pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/", get(get_posts).post(create_post))
        .route("/{id}", get(get_post))
}
