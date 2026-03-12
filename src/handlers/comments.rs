use axum::{Json, extract::State};
use uuid::Uuid;

use crate::{models::comment::Comment, states::server::ServerState};

pub async fn create_comment(
    State(state): State<ServerState>,
    Json(payload): Json<Comment>,
) -> Json<Comment> {
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO comments (id, post_id, user_id, text) VALUES ($1,$2,$3,$4)")
        .bind(id)
        .bind(payload.post_id)
        .bind(payload.user_id)
        .bind(payload.text.clone())
        .execute(&state.db)
        .await
        .unwrap();

    let comment = Comment {
        id,
        post_id: payload.post_id,
        user_id: payload.user_id,
        text: payload.text,
    };

    Json(comment)
}

pub async fn get_comments(State(state): State<ServerState>, post_id: Uuid) -> Json<Vec<Comment>> {
    let comments = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE post_id = $1")
        .bind(post_id)
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(comments)
}

pub async fn get_comment(State(state): State<ServerState>, comment_id: Uuid) -> Json<Comment> {
    let comment = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = $1")
        .bind(comment_id)
        .fetch_one(&state.db)
        .await
        .unwrap();

    Json(comment)
}

pub async fn delete_comment(State(state): State<ServerState>, comment_id: Uuid) {
    sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(comment_id)
        .execute(&state.db)
        .await
        .unwrap();
}

pub async fn update_comment(
    State(state): State<ServerState>,
    comment_id: Uuid,
    Json(payload): Json<Comment>,
) -> Json<Comment> {
    sqlx::query("UPDATE comments SET text = $1 WHERE id = $2")
        .bind(payload.text.clone())
        .bind(comment_id)
        .execute(&state.db)
        .await
        .unwrap();

    let comment = Comment {
        id: comment_id,
        post_id: payload.post_id,
        user_id: payload.user_id,
        text: payload.text,
    };

    Json(comment)
}
