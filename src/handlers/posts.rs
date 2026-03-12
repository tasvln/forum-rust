use axum::extract::Path;
use axum::{Json, extract::State};
use uuid::Uuid;

use crate::{models::post::Post, states::server::ServerState};

pub async fn create_post(
    State(state): State<ServerState>,
    Json(payload): Json<Post>,
) -> Json<Post> {
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO posts (id,title,url,text,user_id) VALUES ($1,$2,$3,$4,$5)")
        .bind(id)
        .bind(payload.title.clone())
        .bind(payload.url.clone())
        .bind(payload.text.clone())
        .bind(payload.user_id)
        .execute(&state.db)
        .await
        .unwrap();

    let post = Post {
        id,
        title: payload.title,
        url: payload.url,
        text: payload.text,
        user_id: payload.user_id,
    };

    Json(post)
}

pub async fn get_post(State(state): State<ServerState>, Path(id): Path<Uuid>) -> Json<Post> {
    let post =
        sqlx::query_as::<_, Post>("SELECT id, title, url, text, user_id FROM posts WHERE id = $1")
            .bind(id)
            .fetch_one(&state.db)
            .await
            .unwrap();

    Json(post)
}

pub async fn get_posts(State(state): State<ServerState>) -> Json<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT id, title, url, text, user_id FROM posts")
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(posts)
}

pub async fn delete_post(State(state): State<ServerState>, Path(id): Path<Uuid>) {
    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .unwrap();
}

pub async fn update_post(
    State(state): State<ServerState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Post>,
) -> Json<Post> {
    sqlx::query("UPDATE posts SET title = $1, url = $2, text = $3 WHERE id = $4")
        .bind(payload.title.clone())
        .bind(payload.url.clone())
        .bind(payload.text.clone())
        .bind(id)
        .execute(&state.db)
        .await
        .unwrap();

    let post = Post {
        id,
        title: payload.title,
        url: payload.url,
        text: payload.text,
        user_id: payload.user_id,
    };

    Json(post)
}
