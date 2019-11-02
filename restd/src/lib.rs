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
use clap::Arg;
use common::log;

pub mod rest;

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

    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author("Christian Saide <me@csaide.dev>")
        .about("REST api for csaide.dev")
        .arg(
            Arg::with_name("log_level")
                .long("log-level")
                .short("l")
                .help("The logging level to use.")
                .possible_values(&["critical", "error", "warn", "info", "debug"])
                .default_value("info")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log_handler")
                .long("log-handler")
                .short("t")
                .help("The logging handler to use.")
                .possible_values(&["stdout", "file"])
                .default_value("stdout")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("log_path")
                .long("log-file")
                .short("f")
                .help("The log file to write to if the 'file' log handler is used.")
                .default_value("")
                .takes_value(true)
                .required_if("log_handler", "file"),
        )
        .arg(
            Arg::with_name("rest_port")
                .long("rest-port")
                .short("p")
                .help("The port to listen on for incoming HTTP requests.")
                .default_value("8080")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rest_addr")
                .long("rest-addr")
                .short("a")
                .help("The address to listen on for incoming HTTP requests.")
                .default_value("0.0.0.0")
                .takes_value(true),
        )
        .get_matches();

    let log_handler = value_t!(matches, "log_handler", log::Handler).unwrap_or_else(|e| {
        e.exit();
    });

    let log_level = value_t!(matches, "log_level", log::Level).unwrap_or_else(|e| {
        e.exit();
    });

    let log_path = value_t!(matches, "log_path", String).unwrap_or_else(|e| {
        e.exit();
    });

    let rest_port = value_t!(matches, "rest_port", u16).unwrap_or_else(|e| {
        e.exit();
    });

    let rest_addr = value_t!(matches, "rest_addr", String).unwrap_or_else(|e| {
        e.exit();
    });

    let listen_addr = format!("{}:{}", rest_addr, rest_port);

    let logger_cfg = log::config::Config {
        handler: log_handler,
        level: log_level,
        path: log_path,
    };

    let root_logger = match log::new(&logger_cfg, crate_name!(), crate_version!()) {
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
