use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    pub user_id: Uuid,
}
