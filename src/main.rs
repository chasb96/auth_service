use tokio::net::TcpListener;
use auth_service::{util::log_unwrap::LogUnwrap, routes};
use ::axum::{serve, Router};

#[tokio::main]
async fn main() {
    env_logger::init();

    let router = routes::routes(Router::new());
    let listener = TcpListener::bind("0.0.0.0:80").await.log_unwrap();

    serve(listener, router).await.log_unwrap();
}