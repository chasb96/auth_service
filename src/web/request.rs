use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetPasswordRequest {
    pub id: i32,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyJwtRequest {
    pub jwt: String,
}