// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Crates
#[macro_use]
extern crate slog;
#[macro_use]
extern crate clap;

extern crate actix_service;
extern crate actix_web;
extern crate chrono;
extern crate common;
extern crate futures;
extern crate serde;
extern crate serde_json;

// Standard usings
use actix_web::{App, HttpServer};
use common::log;
use structopt::StructOpt;

pub mod rest;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    global_settings = &[clap::AppSettings::DeriveDisplayOrder],
    author = "Christian Saide <me@csaide.dev>",
    about = "REST api for csaide.dev"
)]
struct Config {
    #[structopt(flatten)]
    log_config: log::Config,

    #[structopt(flatten)]
    rest_config: rest::Config,
}

pub fn run() -> i32 {
    let setup_logger = log::new(
        &log::config::Config {
            handler: log::Handler::Stdout,
            level: log::Level::Crit,
            path: String::from(""),
        },
        crate_name!(),
        crate_version!(),
    )
    .unwrap();

    let cfg = Config::from_args();

    let listen_addr = format!("{}:{}", cfg.rest_config.addr, cfg.rest_config.port);

    let root_logger = match log::new(&cfg.log_config, crate_name!(), crate_version!()) {
        Ok(root_logger) => root_logger,
        Err(e) => {
            crit!(
                setup_logger,
                "Failed to generate logger based on supplied configuration.";
                e
            );
            return 1;
        }
    };

    info!(
        root_logger,
        "Starting HTTP Server.";
        o!("addr" => &listen_addr)
    );

    let logging = rest::logger::Logging::new(root_logger.new(o!("logger" => "rest")));
    HttpServer::new(move || App::new().wrap(logging.clone()).configure(rest::configure))
        .bind(listen_addr)
        .unwrap()
        .run()
        .unwrap();

    return 0;
}
