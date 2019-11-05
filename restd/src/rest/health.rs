// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Health {
    #[serde(skip_serializing, skip_deserializing)]
    status: StatusCode,
    alive: bool,
}

impl Responder for Health {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        Ok(HttpResponse::build(self.status).json2(&self))
    }
}

pub fn endpoint() -> impl Responder {
    Health {
        status: StatusCode::OK,
        alive: true,
    }
}
