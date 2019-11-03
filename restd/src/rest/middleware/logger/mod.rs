use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use serde;
use serde::ser::SerializeStruct;
use slog::info;
use std::result;

#[derive(Debug, Clone, SerdeValue)]
pub struct RequestLog {
    route: String,
    method: String,
    uri: String,
}

impl serde::Serialize for RequestLog {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sv = serializer.serialize_struct("RequestLog", 3)?;
        sv.serialize_field("route", &self.route)?;
        sv.serialize_field("method", &self.method)?;
        sv.serialize_field("uri", &self.uri)?;
        sv.end()
    }
}

impl slog::KV for RequestLog {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("req", self)
    }
}

#[derive(Debug, Clone, SerdeValue)]
pub struct ResponseLog {
    response_time: i64,
    status: u16,
}

impl serde::Serialize for ResponseLog {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut sv = serializer.serialize_struct("ResponseLog", 2)?;
        sv.serialize_field("response_time_us", &self.response_time)?;
        sv.serialize_field("status", &self.status)?;
        sv.end()
    }
}

impl slog::KV for ResponseLog {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("res", self)
    }
}

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

            let request_log = RequestLog {
                route: req.path().to_string(),
                method: req.method().to_string(),
                uri: req.uri().to_string(),
            };

            let response_log = ResponseLog {
                response_time: duration.num_microseconds().unwrap(),
                status: res.status().as_u16(),
            };

            info!(logger, "handled request";
                request_log,
                response_log
            );
            Ok(res)
        }))
    }
}
