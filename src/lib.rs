use std::str::FromStr;

use service_invoker::{ServiceResult, Client, ServiceRequest, Method, HeaderMap, HeaderValue, AUTHORIZATION};
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

pub struct AuthServiceConfig {
    pub base_url: Url,
    pub token: String,
}

pub struct AuthService<'a> {
    client: Client<'a>,
    base_url: String,
    token_header: HeaderValue,
}

impl<'a> AuthService<'a> {
    pub async fn authenticate(&self, request: AuthenticateRequest) -> ServiceResult<AuthenticateResponse> {
        let route = Url::from_str(&format!("{}{}", self.base_url, "/authenticate")).unwrap();
        let mut headers = HeaderMap::new();

        headers.append(AUTHORIZATION, self.token_header.clone());

        ServiceRequest::builder()
            .with_url(route)
            .with_headers(headers)
            .with_method(Method::POST)
            .with_body(request)
            .build()
            .send(&self.client)
            .await
    }

    pub async fn verify_jwt(&self, request: VerifyJwtRequest) -> ServiceResult<VerifyJwtResponse> {
        let route = Url::from_str(&format!("{}{}", self.base_url, "/verify_jwt")).unwrap();
        let mut headers = HeaderMap::new();

        headers.append(AUTHORIZATION, self.token_header.clone());

        ServiceRequest::builder()
            .with_url(route)
            .with_headers(headers)
            .with_method(Method::POST)
            .with_body(request)
            .build()
            .send(&self.client)
            .await
    }

    pub async fn set_password(&self, request: SetPasswordRequest) -> ServiceResult<()> {
        let route = Url::from_str(&format!("{}{}", self.base_url, "/set_password")).unwrap();
        let mut headers = HeaderMap::new();

        headers.append(AUTHORIZATION, self.token_header.clone());

        ServiceRequest::builder()
            .with_url(route)
            .with_headers(headers)
            .with_method(Method::POST)
            .with_body(request)
            .build()
            .send(&self.client)
            .await
    }
}

impl<'a> From<&AuthServiceConfig> for AuthService<'a> {
    fn from(value: &AuthServiceConfig) -> Self {
        let token_header = HeaderValue::from_str(&format!("TOKEN {}", &value.token)).unwrap();
        
        Self {
            client: Client::default(),
            base_url: value.base_url.to_string(),
            token_header,
        }
    }
}