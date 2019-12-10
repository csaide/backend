// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use super::RequestID;
use gotham::state::{FromState, State};
use hyper::{body::Payload, Body, Method, Response, Uri, Version};
use serde::Serialize;

#[derive(Debug, Clone, SerdeValue, Serialize)]
pub struct Req {
    pub uri: String,
    pub method: String,
    pub version: String,
    pub id: String,
}

impl Req {
    pub fn new(state: &State) -> Self {
        Self {
            uri: Uri::borrow_from(state).path().to_owned(),
            method: Method::borrow_from(state).as_str().to_owned(),
            version: format!("{:?}", Version::borrow_from(state)),
            id: RequestID::borrow_from(state).as_str().to_owned(),
        }
    }
}

impl slog::KV for Req {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("req", self)
    }
}

#[derive(Debug, Clone, SerdeValue, Serialize)]
pub struct Res {
    pub latency: f64,
    pub status: u16,
    pub size: u64,
}

impl Res {
    pub fn new(resp: &Response<Body>, latency: f64) -> Self {
        Self {
            latency: latency,
            status: resp.status().as_u16(),
            size: resp.body().content_length().unwrap_or(0),
        }
    }
}

impl slog::KV for Res {
    fn serialize(&self, _: &slog::Record, serializer: &mut dyn slog::Serializer) -> slog::Result {
        serializer.emit_serde("res", self)
    }
}
