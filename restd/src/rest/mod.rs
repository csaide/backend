// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::Router;

mod config;
mod error;
mod health;
mod metrics;
mod middleware;

pub use config::Config;

fn router(root_logger: &slog::Logger) -> Router {
    let handler = middleware::telemetry::Handler {
        logger: root_logger.new(o!("logger" => "rest")),
    };

    let pipeline = new_pipeline().add(handler).build();

    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.get("/metrics").to(metrics::endpoint);
        route.get("/health").to(health::endpoint);
    })
}

pub fn server(cfg: &config::Config, root_logger: &slog::Logger) -> error::Result<()> {
    let listen_addr = format!("{}:{}", cfg.addr, cfg.port);
    let rt = router(&root_logger);

    info!(root_logger, "Starting HTTP Rest API Server!"; "addr" => &listen_addr);
    gotham::start_with_num_threads(listen_addr, rt, cfg.workers);
    Ok(())
}
