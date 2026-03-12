use crate::ServerState;
use crate::auth::jwt::create_token;
use crate::auth::password::{hash_password, verify_password};
use axum::{Json, extract::State};
use sqlx::Row;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(State(state): State<ServerState>, Json(payload): Json<RegisterRequest>) {
    let password_hash = hash_password(&payload.password);
    let user_id = Uuid::new_v4();

    sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ($1,$2,$3)")
        .bind(user_id)
        .bind(payload.username.clone())
        .bind(password_hash)
        .execute(&state.db)
        .await
        .unwrap();
}

pub async fn login(
    State(state): State<ServerState>,
    Json(payload): Json<LoginRequest>,
) -> Json<String> {
    let user = sqlx::query("SELECT id,password_hash FROM users WHERE username = $1")
        .bind(payload.username.clone())
        .fetch_one(&state.db)
        .await
        .unwrap();

    let password_hash: String = user.get("password_hash");
    let user_id: Uuid = user.get("id");

    if verify_password(&password_hash, &payload.password) {
        let token = create_token(&user_id.to_string(), &state.jwt_secret);
        Json(token)
    } else {
        Json("Invalid credentials".to_string())
    }
}

pub async fn reset_password(State(state): State<ServerState>, Json(payload): Json<LoginRequest>) {
    let password_hash = hash_password(&payload.password);

    sqlx::query("UPDATE users SET password_hash = $1 WHERE username = $2")
        .bind(password_hash)
        .bind(payload.username.clone())
        .execute(&state.db)
        .await
        .unwrap();
}
