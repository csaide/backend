// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::HttpResponse;
use prometheus::{self, Encoder, TextEncoder};

pub fn endpoint() -> HttpResponse {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer.clone()).unwrap();
    HttpResponse::Ok().content_type("text/plain").body(body)
}
