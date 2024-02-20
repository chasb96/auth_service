use axum::{http::StatusCode, Json};
use crate::{authorizer::{jwt, password::{generate_password_hash, verify_password}}, axum::extractors::{AuthorizerExtractor, UserStoreExtractor}, users::{claims_user::ClaimsUser, UserStore}, util::or_status_code::{OrBadRequest, OrInternalServerError}};
use super::{request::{AuthenticateRequest, SetPasswordRequest, VerifyJwtRequest}, response::{AuthenticateResponse, VerifyJwtResponse}};

pub async fn authenticate(
    _: AuthorizerExtractor,
    user_store: UserStoreExtractor,
    Json(request): Json<AuthenticateRequest>
) -> Result<Json<AuthenticateResponse>, StatusCode> {
    let user = user_store
        .get_by_username(&request.username)
        .await
        .or_internal_server_error()?
        .or_bad_request()?;

    if !verify_password(&request.password, &user.password_hash).or_internal_server_error()? {
        return Err(StatusCode::UNAUTHORIZED)
    }

    jwt::generate_jwt(ClaimsUser::from(user))
        .or_internal_server_error()
        .map(|token| Json(
            AuthenticateResponse {
                jwt: token,
            }
        ))
}

pub async fn verify_jwt(
    _: AuthorizerExtractor,
    Json(request): Json<VerifyJwtRequest>,
) -> Result<Json<VerifyJwtResponse>, StatusCode> {
    jwt::verify_jwt(request.jwt)
        .map(|user: ClaimsUser| Json(VerifyJwtResponse {
            id: user.id,
        }))
        .map_err(|err| match err {
            jwt::ValidateJwtError::HmacKey(_) => StatusCode::INTERNAL_SERVER_ERROR,
            jwt::ValidateJwtError::Verify(_) => StatusCode::INTERNAL_SERVER_ERROR,
            jwt::ValidateJwtError::Claims(_) => StatusCode::UNAUTHORIZED,
        })
}

pub async fn set_password(
    _: AuthorizerExtractor,
    user_store: UserStoreExtractor,
    Json(request): Json<SetPasswordRequest>
) -> Result<StatusCode, StatusCode> {
    let password_hash = generate_password_hash(&request.password)
        .or_internal_server_error()?;

    user_store
        .set_password_hash(request.id, &password_hash)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .or_internal_server_error()
}