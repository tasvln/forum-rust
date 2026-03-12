use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod auth;
mod handlers;
mod models;
mod routes;
mod states;

use states::server::ServerState;

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

    // get database url
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to postgres (Supabase)
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // load jwt secret
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // create shared state
    let state = ServerState { db, jwt_secret };

    // build router
    let app = routes::create_router().with_state(state);

    // start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
