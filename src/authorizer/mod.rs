mod token;
mod hmac;
pub mod jwt;
pub mod password;

pub use token::TokenAuthorizer;

pub trait Authorizer {
    async fn verify<'a>(&self, token: &'a str) -> bool;
}