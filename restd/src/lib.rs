// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Crates
#[macro_use]
extern crate slog;
#[macro_use]
extern crate clap;
extern crate common;

// Standard usings
use clap::{App, Arg};
use common::log;

pub fn run() -> i32 {
    let setup_logger = log::new(
        &log::config::Config {
            handler: log::Handler::Stdout,
            level:   log::Level::Crit,
            path:    String::from(""),
        },
        crate_name!(),
        crate_version!(),
    )
    .unwrap();

    let matches = App::new(crate_name!())
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

    let logger_cfg = log::config::Config {
        handler: log_handler,
        level:   log_level,
        path:    log_path,
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

    error!(root_logger, "Error message!");
    warn!(root_logger, "Warning message!");
    info!(
        root_logger,
        "Initialized system, and ready to accept queries.";
        o!("hello" => "world")
    );
    debug!(root_logger, "Debug message!");

    return 0;
}
