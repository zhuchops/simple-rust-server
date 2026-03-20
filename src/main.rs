use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(pong));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pong() -> &'static str {
    "pong"
}
