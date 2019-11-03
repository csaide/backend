// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::web;

pub mod hello;
pub mod model;

// this function could be located in different module
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").configure(hello::configure));
}
