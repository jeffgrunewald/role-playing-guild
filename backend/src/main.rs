use axum::{Router, routing::get};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:data.db?mode=rwc")
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/health", get(health))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}
