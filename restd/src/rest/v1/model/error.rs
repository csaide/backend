// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt;

pub type Result<T> = actix_web::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub status: u16,
    pub retryable: bool,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_json = serde_json::to_string(self).unwrap();
        write!(f, "{}", err_json)
    }
}

impl ResponseError for Error {
    fn render_response(&self) -> HttpResponse {
        let status =
            StatusCode::from_u16(self.status).expect("Invalid status code specified in error.");
        HttpResponse::build(status).json2(self)
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
    Error {
        status: 403,
        retryable: false,
        message: String::from("Unauthorized access"),
    }
}

pub fn internal_error() -> Error {
    Error {
        status: 500,
        retryable: false,
        message: String::from("Internal server error."),
    }
}
