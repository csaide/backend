// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Standard usings
use actix_web::{App, HttpServer};

pub mod config;
pub mod error;
pub mod middleware;
pub mod v1;

pub use config::Config;

use middleware::logger;

pub fn server(cfg: &config::Config, root_logger: &slog::Logger) -> error::Result<()> {
    let listen_addr = format!("{}:{}", cfg.addr, cfg.port);
    let logging = logger::Logging::new(root_logger.new(o!("logger" => "rest")));

    info!(root_logger, "Starting HTTP Server."; o!("addr" => &listen_addr));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(logging.clone())
            .wrap(actix_web::middleware::NormalizePath)
            .configure(v1::configure)
    });

    let server = match server.bind(&listen_addr) {
        Ok(server) => server,
        Err(e) => {
            return Err(error::Error::BindError {
                addr: listen_addr,
                err: std::sync::Arc::new(e),
            })
        }
    };

    match server.run() {
        Ok(()) => Ok(()),
        Err(e) => Err(error::Error::RunError {
            err: std::sync::Arc::new(e),
        }),
    }
}
