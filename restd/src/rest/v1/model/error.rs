// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = actix_web::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    #[serde(skip_serializing, skip_deserializing)]
    pub status: StatusCode,
    pub retryable: bool,
    pub message: String,
}

impl Error {
    pub fn to_actix(&self) -> actix_web::Error {
        actix_web::Error::from(self.clone())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_json = serde_json::to_string(self).unwrap();
        write!(f, "{}", err_json)
    }
}

impl ResponseError for Error {
    fn render_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).json2(self)
    }
}

impl Responder for Error {
    type Error = actix_web::Error;
    type Future = actix_web::Result<HttpResponse, actix_web::Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        // Create response and set content type
        Ok(self.render_response())
    }
}

pub fn unauthorized() -> Error {
    let status = StatusCode::UNAUTHORIZED;
    Error {
        status: status,
        retryable: false,
        message: format!("{}", status),
    }
}

pub fn forbidden() -> Error {
    let status = StatusCode::FORBIDDEN;
    Error {
        status: status,
        retryable: false,
        message: format!("{}", status),
    }
}

pub fn internal_error() -> Error {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    Error {
        status: status,
        retryable: false,
        message: format!("{}", status),
    }
}

pub fn not_implemented() -> Error {
    let status = StatusCode::NOT_IMPLEMENTED;
    Error {
        status: status,
        retryable: false,
        message: format!("{}", status),
    }
}
