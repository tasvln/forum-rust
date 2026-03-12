use crate::ServerState;
use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::comments::{create_comment, get_comments};

pub fn routes() -> Router<ServerState> {
    Router::new().route("/", post(create_comment))
    // .route("/:post_id", get(get_comments))
}
