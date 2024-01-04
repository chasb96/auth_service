use serde::Deserialize;
use service_invoker::{ServiceResult, Client, ServiceRequest, Method, AUTHORIZATION};
use url::Url;
use web::{request::{AuthenticateRequest, VerifyJwtRequest, SetPasswordRequest}, response::{AuthenticateResponse, VerifyJwtResponse}};

mod axum;
mod configuration;
mod routes;
mod util;

mod authorizer;
mod data_stores;
mod web;
mod users;

#[derive(Deserialize)]
pub struct AuthServiceConfig {
    pub base_url: String,
    pub token: String,
}

pub struct AuthService<'a> {
    client: Client<'a>,
    base_url: Url,
    token_header: String,
}

impl<'a> AuthService<'a> {
    pub async fn authenticate(&self, request: AuthenticateRequest) -> ServiceResult<AuthenticateResponse> {
        ServiceRequest::builder()
            .with_base_url(self.base_url.clone())
            .with_path("/authenticate")
            .with_header_str(AUTHORIZATION, &self.token_header)?
            .with_method(Method::POST)
            .with_body(request)
            .build()
            .send(&self.client)
            .await
    }

    pub async fn verify_jwt(&self, request: VerifyJwtRequest) -> ServiceResult<VerifyJwtResponse> {
        ServiceRequest::builder()
            .with_base_url(self.base_url.clone())
            .with_path("/verify_jwt")
            .with_header_str(AUTHORIZATION, &self.token_header)?
            .with_method(Method::POST)
            .with_body(request)
            .build()
            .send(&self.client)
            .await
    }

    pub async fn set_password(&self, request: SetPasswordRequest) -> ServiceResult<()> {
        ServiceRequest::builder()
            .with_base_url(self.base_url.clone())
            .with_path("/set_password")
            .with_header_str(AUTHORIZATION, &self.token_header)?
            .with_method(Method::POST)
            .with_body(request)
            .build()
            .send(&self.client)
            .await
    }
}

impl<'a> From<&AuthServiceConfig> for AuthService<'a> {
    fn from(value: &AuthServiceConfig) -> Self {
        Self {
            client: Client::default(),
            base_url: Url::parse(&value.base_url).expect("Invalid authentication host configured"),
            token_header: format!("TOKEN {}", &value.token),
        }
    }
}