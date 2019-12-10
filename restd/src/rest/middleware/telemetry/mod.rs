// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use chrono::Utc;
use futures::{future, Future};
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;
use hyper::header::CONTENT_LENGTH;
use std::io;

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
                let end_time = Utc::now();
                let duration = end_time - start_time;

                let latency = duration.num_microseconds().unwrap() as f64 / 1000.0;

                let req = records::Request::from(&state);

                let status = response.status().as_u16();
                let size = response
                    .headers()
                    .get(CONTENT_LENGTH)
                    .map(|len| len.to_str().unwrap())
                    .unwrap_or("0");

                let res = records::Response {
                    latency_ms: latency,
                    status: status,
                    size: size.to_owned(),
                };

                let labels = [
                    req.uri.as_str(),
                    req.method.as_str(),
                    &format!("{}", status),
                ];
                metrics::REQUEST_COUNTER.with_label_values(&labels).inc();
                metrics::LATENCY_HISTOGRAM
                    .with_label_values(&labels)
                    .observe(latency);

                if status < 400 {
                    debug!(self.logger, "Successfully handled request."; req, res);
                } else if status < 500 {
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
