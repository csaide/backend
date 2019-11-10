// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::web;

use super::middleware::Authenticator;
use super::model::{error, hello::HelloResponse};

pub fn get() -> error::Result<HelloResponse> {
    Ok(HelloResponse {
        message: String::from("Hello World!!!"),
    })
}

pub fn get_from(user: web::Path<String>) -> error::Result<HelloResponse> {
    Ok(HelloResponse {
        message: format!("Hello {}!", user),
    })
}

pub fn error() -> error::Result<HelloResponse> {
    Err(error::internal_error())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    let authenticator = Authenticator {};
    cfg.service(
        web::scope("/admin")
            .wrap(authenticator)
            .route("/", web::get().to(get))
            .route("/error", web::get().to(error))
            .route("/{user}", web::get().to(get_from)),
    );
}
