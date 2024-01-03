use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AuthenticateResponse {
    pub jwt: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyJwtResponse {
    pub id: i32,
}