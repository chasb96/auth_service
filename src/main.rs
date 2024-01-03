use tokio::net::TcpListener;
use util::log_unwrap::LogUnwrap;
use ::axum::{serve, Router};

mod axum;
mod configuration;
mod routes;
mod util;

mod authorizer;
mod data_stores;
mod web;
mod users;

#[tokio::main]
async fn main() {
    env_logger::init();

    let router = routes::routes(Router::new());
    let listener = TcpListener::bind("0.0.0.0:80").await.log_unwrap();

    serve(listener, router).await.log_unwrap();
}