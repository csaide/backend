// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use gotham::helpers::http::response::create_response;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use prometheus::{self, Encoder, TextEncoder};

pub fn endpoint(state: State) -> (State, Response<Body>) {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer.clone()).unwrap();
    let resp = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, body);

    (state, resp)
}
