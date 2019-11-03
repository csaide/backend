use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use slog::info;

// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
#[derive(Clone)]
pub struct Logging {
    logger: std::sync::Arc<slog::Logger>,
}

impl Logging {
    pub fn new(logger: slog::Logger) -> Logging {
        Logging {
            logger: std::sync::Arc::new(logger),
        }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service,
            logger: self.logger.clone(),
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
    logger: std::sync::Arc<slog::Logger>,
}

impl<S, B> Service for LoggingMiddleware<S>
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
            info!(logger, "handled request";
                "responseTime" => duration.num_microseconds(),
                "uri" => %req.uri(),
                "route" => req.path(),
                "method" => %req.method(),
                "statusCode" => res.status().as_u16()
            );
            Ok(res)
        }))
    }
}
