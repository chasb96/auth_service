use axum::{Router, routing::{post, get}};
use crate::{web::routes::{authenticate, verify_jwt, set_password}, health};

#[allow(dead_code)]
pub fn routes(router: Router) -> Router {
    router
        .route("/health", get(health::health))
        .route("/authenticate", post(authenticate))
        .route("/verify_jwt", post(verify_jwt))
        .route("/set_password", post(set_password))
}