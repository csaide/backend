// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

#[macro_use]
extern crate slog;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate slog_derive;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate gotham_derive;

use common::log;
use structopt::StructOpt;

mod rest;

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
    let cfg = Config::from_args();

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

    let root_logger = match log::new(&cfg.log_config, crate_name!(), crate_version!()) {
        Ok(root_logger) => root_logger,
        Err(e) => {
            crit!(setup_logger, "Failed to generate logger based on supplied configuration."; e);
            return 1;
        }
    };

    rest::server(&cfg.rest_config, &root_logger);
    info!(root_logger, "Server shutdown.");
    return 0;
}
