use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{HitCounter, PASSWORD_COOKIE};
use crate::ServiceError;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::Responder;
use rocket::State;
use serde::Serialize;
use std::str::FromStr;

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[error("Invalid API key format")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String)
}

#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),

    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    User(Json<String>),

    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<ApiKeyError>)
}

#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn to_base64(&self) -> String {
        base64::encode(self.0.as_slice())
    }
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyError;
    fn from_str(key: &str) -> Result<Self, Self::Err> {
        base64::decode(key)
            .map(ApiKey)
            .map_err(|e| Self::Err::DecodeError(e.to_string()))
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Clip(c) => Self::User(Json(format!("clip parsing error: {:?}", c))),
            ServiceError::NotFound => Self::User(Json("entity not found".to_owned())),
            ServiceError::Data(_) => Self::User(Json("a server error occurred".to_owned())),
            ServiceError::PermissionError(msg) => Self::User(Json(msg))
        }
    }
}