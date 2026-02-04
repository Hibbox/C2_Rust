use axum::{ Router, routing::{get, post, put, delete} };
use sqlx::{ postgres::PgPoolOptions };
use tokio;
use std::env;

mod resource;
use resource::*;




#[tokio::main]

async fn main() {
    let db_url = env::var("DATABASE_URL").expect("-_- DATABASE_URL must be set for working -_-");
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect on url");
    sqlx::migrate!().run(&pool).await.expect("tables doesn't create");

    let app = Router::new()
        .route("/task", get(get_task).post(add_task))
        .route("/results", get(get_result).post(add_result))
        .route("/users", post(create_user).get(list_users))
        .route("/user/:id", get(get_users_id).put(update_user).delete(delete_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server en ecoute :)...");
    axum::serve(listener,app).await.unwrap();



}