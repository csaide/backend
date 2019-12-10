// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use gotham::state::{FromState, State};
use hyper::{Method, Uri, Version};
use serde::Serialize;

#[derive(Debug, Clone, SerdeValue, Serialize)]
pub struct Request {
    pub uri: String,
    pub method: String,
    pub version: String,
}

impl slog::KV for Request {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("req", self)
    }
}

impl From<&State> for Request {
    fn from(state: &State) -> Request {
        Request {
            uri: Uri::borrow_from(state).path().to_owned(),
            method: Method::borrow_from(state).as_str().to_owned(),
            version: format!("{:?}", Version::borrow_from(state)),
        }
    }
}

#[derive(Debug, Clone, SerdeValue, Serialize)]
pub struct Response {
    pub latency_ms: f64,
    pub status: u16,
    pub size: String,
}

impl slog::KV for Response {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("res", self)
    }
}
