// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

// Standard usings
use actix_web::{guard, web, App, HttpServer};

mod config;
mod error;
mod health;
mod metrics;
mod middleware;
mod v1;

pub use config::Config;

use middleware::telemetry;

pub fn server(cfg: &config::Config, root_logger: &slog::Logger) -> error::Result<()> {
    let listen_addr = format!("{}:{}", cfg.addr, cfg.port);
    let logging = telemetry::Handler::new(root_logger.new(o!("logger" => "rest")));

    info!(root_logger, "Starting HTTP Server."; o!("addr" => &listen_addr));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(logging.clone())
            .wrap(actix_web::middleware::NormalizePath)
            .route(
                "/metrics",
                web::route().guard(guard::Get()).to(metrics::endpoint),
            )
            .route(
                "/health",
                web::route().guard(guard::Get()).to(health::endpoint),
            )
            .configure(v1::configure)
    });

    let server = server.bind(&listen_addr).or_else(|e| {
        Err(error::Error::BindError {
            addr: listen_addr,
            err: std::sync::Arc::new(e),
        })
    })?;

    server.run().or_else(|e| {
        Err(error::Error::RunError {
            err: std::sync::Arc::new(e),
        })
    })
}
