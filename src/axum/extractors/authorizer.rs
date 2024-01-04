use axum::{async_trait, extract::FromRequestParts, http::{StatusCode, request::Parts, header::AUTHORIZATION}};
use crate::{authorizer::{TokenAuthorizer, Authorizer}, util::or_status_code::OrStatusCode};

pub struct AuthorizerExtractor;

#[async_trait]
impl<T> FromRequestParts<T> for AuthorizerExtractor {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(parts: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let auth_header = match parts.headers.get(AUTHORIZATION) {
            Some(header_value) => header_value
                .to_str()
                .or_status_code(StatusCode::BAD_REQUEST)?
                .split_once(' ')
                .or_status_code(StatusCode::UNAUTHORIZED)?,
            None => return Err(StatusCode::UNAUTHORIZED),
        };

        let token = match auth_header.0.to_uppercase().as_ref() {
            "TOKEN" => auth_header.1,
            _ => return Err(StatusCode::BAD_REQUEST),
        };

        if TokenAuthorizer::default().verify(token).await {
            Ok(AuthorizerExtractor)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}