// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use chrono::Utc;
use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;
use std::io;

use super::request_id::RequestID;

mod metrics;
mod records;

pub struct Handler {
    pub logger: slog::Logger,
}

impl NewMiddleware for Handler {
    type Instance = Self;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(Handler {
            logger: self.logger.clone(),
        })
    }
}

impl Middleware for Handler {
    fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        let start_time = Utc::now();

        let f = chain(state).and_then(move |(state, response)| {
            {
                let duration = Utc::now() - start_time;
                let latency = duration.num_microseconds().unwrap_or(1) as f64 / 1_000_000.0;

                let req = records::Req::new(&state);
                let res = records::Res::new(&response, latency);

                let labels = [
                    req.uri.as_str(),
                    req.method.as_str(),
                    &res.status.to_string(),
                ];

                metrics::REQUEST_COUNTER.with_label_values(&labels).inc();
                metrics::LATENCY_HISTOGRAM
                    .with_label_values(&labels)
                    .observe(latency);
                metrics::RESPONSE_SIZE_HISTOGRAM
                    .with_label_values(&labels)
                    .observe(res.size as f64);

                if res.status < 400 {
                    debug!(self.logger, "Successfully handled request."; req, res);
                } else if res.status < 500 {
                    warn!(self.logger, "Client error during request."; req, res);
                } else {
                    error!(self.logger, "Server error during request."; req, res);
                }
            };
            future::ok((state, response))
        });

        Box::new(f)
    }
}
