// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::{web, Responder};

use super::model::hello::HelloResponse;

pub fn get() -> impl Responder {
    HelloResponse {
        message: String::from("Hello World!!!"),
    }
}

pub fn get_from(user: web::Path<String>) -> impl Responder {
    HelloResponse {
        message: format!("Hello {}!", user),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hello")
            .route("", web::get().to(get))
            .route("/", web::get().to(get))
            .route("/{user}", web::get().to(get_from)),
    );
}
