pub mod auth;
pub mod comments;
pub mod posts;

use axum::Router;

use crate::ServerState;

pub fn create_router() -> Router<ServerState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/posts", posts::routes())
        .nest("/comments", comments::routes())
}
