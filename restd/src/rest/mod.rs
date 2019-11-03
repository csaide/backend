// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Standard usings
use actix_web::{guard, web, Error, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub mod config;
pub mod logger;

pub use config::Config;

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

// Responder
impl Responder for HelloResponse {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn get() -> impl Responder {
    HelloResponse {
        message: String::from("Hello World!!!"),
    }
}

fn get_from(user: web::Path<String>) -> impl Responder {
    HelloResponse {
        message: format!("Hello {}!", user),
    }
}

// this function could be located in different module
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").guard(guard::Get()).to(get));
    cfg.service(
        web::resource("/hello/{user}")
            .guard(guard::Get())
            .to(get_from),
    );
}
