use actix_web::HttpRequest;
use serde::Serialize;

#[derive(Debug, Clone, SerdeValue, Serialize)]
pub struct Request {
    pub route: String,
    pub method: String,
    pub uri: String,
}

impl slog::KV for Request {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("req", self)
    }
}

impl From<&HttpRequest> for Request {
    fn from(req: &HttpRequest) -> Request {
        Request {
            route: req.path().to_string(),
            method: req.method().to_string(),
            uri: req.uri().to_string(),
        }
    }
}

#[derive(Debug, Clone, SerdeValue, Serialize)]
pub struct Response {
    pub latency_us: i64,
    pub status: u16,
}

impl slog::KV for Response {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("res", self)
    }
}
