use axum::{
    routing::{get, post}, 
    Error, 
    Router,
    response::IntoResponse
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_users() -> Result<(), Error> {
    todo!()
}