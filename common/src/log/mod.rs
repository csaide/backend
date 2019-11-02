// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Crates
extern crate serde;
extern crate slog_async;
extern crate slog_json;

// Super usings
use super::slog;

// Standard usings
use slog::Drain;
use std::fs::OpenOptions;
use std::io;
use std::result;

// Local modules
pub mod config;
pub mod error;
pub mod handler;
pub mod level;

// Local module usings
pub use config::Config;
pub use error::{Error, Result};
pub use handler::Handler;
pub use level::Level;

struct LevelFilter<D> {
    drain: D,
    level: slog::Level,
}

impl<D> Drain for LevelFilter<D>
where
    D: Drain,
{
    type Err = Option<D::Err>;
    type Ok = Option<D::Ok>;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        if record.level().is_at_least(self.level) {
            self.drain.log(record, values).map(Some).map_err(Some)
        } else {
            Ok(None)
        }
    }
}

pub fn new(
    cfg: &config::Config,
    app: &'static str,
    version: &'static str,
) -> error::Result<slog::Logger> {
    match cfg.handler {
        Handler::File => {
            let file = match OpenOptions::new().create(true).append(true).open(&cfg.path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(error::Error::IOError {
                        path: cfg.path.clone(),
                        err: std::sync::Arc::new(e),
                    });
                }
            };

            let drain = slog_json::Json::new(file).add_default_keys().build().fuse();
            let drain = LevelFilter {
                drain: drain,
                level: cfg.level.to_slog(),
            }
            .fuse();
            let drain = slog_async::Async::new(drain).build().fuse();

            Ok(slog::Logger::root(
                drain,
                o!("app" => app, "ver" => version),
            ))
        }
        Handler::Stdout => {
            let drain = slog_json::Json::new(io::stdout())
                .add_default_keys()
                .build()
                .fuse();
            let drain = LevelFilter {
                drain: drain,
                level: cfg.level.to_slog(),
            }
            .fuse();

            let drain = slog_async::Async::new(drain).build().fuse();

            Ok(slog::Logger::root(
                drain,
                o!("app" => app, "ver" => version),
            ))
        }
    }
}
