mod user;
use axum::{
    routing::{get, post}, 
    Error, 
    Router,
    response::IntoResponse
};
use user::User;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users", get(get_users()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_users() -> Result<Vec<User>, Error> {
    todo!()
}