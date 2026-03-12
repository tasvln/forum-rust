use crate::states::server::ServerState;
use axum::{Json, extract::State};
use sqlx::Row;
use uuid::Uuid;

pub async fn get_user(State(state): State<ServerState>, user_id: Uuid) -> Json<Option<String>> {
    let user = sqlx::query("SELECT username FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if let Some(row) = user {
        let username: String = row.get("username");
        Json(Some(username))
    } else {
        Json(None)
    }
}
