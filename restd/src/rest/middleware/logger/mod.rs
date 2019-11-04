use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use slog::{debug, error, warn};

mod messages;

#[derive(Clone)]
pub struct Logger {
    logger: std::sync::Arc<slog::Logger>,
}

impl Logger {
    pub fn new(logger: slog::Logger) -> Logger {
        Logger {
            logger: std::sync::Arc::new(logger),
        }
    }
}

impl<S, B> Transform<S> for Logger
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggerMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerMiddleware {
            service,
            logger: self.logger.clone(),
        })
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
    logger: std::sync::Arc<slog::Logger>,
}

impl<S, B> Service for LoggerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start_time = chrono::Utc::now();
        let logger = self.logger.clone();

        Box::new(self.service.call(req).and_then(move |res| {
            let req = res.request();
            let end_time = chrono::Utc::now();
            let duration = end_time - start_time;
            let status = res.status().as_u16();

            let req_log = messages::Request::from(req);
            let res_log = messages::Response {
                latency_us: duration.num_microseconds().unwrap(),
                status: status,
            };

            if status < 400 {
                debug!(logger, "Successfully handled request."; req_log, res_log);
            } else if status < 500 {
                warn!(logger, "Client error during request.";  req_log, res_log)
            } else {
                error!(logger, "Server error during request.";  req_log, res_log)
            }

            Ok(res)
        }))
    }
}
