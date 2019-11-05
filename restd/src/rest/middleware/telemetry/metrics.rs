// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_web::HttpResponse;
use prometheus::{self, Encoder, TextEncoder};

lazy_static! {
    pub static ref REQUEST_COUNTER: prometheus::IntCounterVec = register_int_counter_vec!(
        "restd_rest_requests",
        "Number of requests handled by route/method/status.",
        &["route", "method", "status"]
    )
    .unwrap();
    pub static ref LATENCY_HISTOGRAM: prometheus::HistogramVec = register_histogram_vec!(
        "restd_rest_latency_us",
        "Latency microsecond resolution buckets of requests handled by route/method/status.",
        &["route", "method", "status"],
        vec![
            1000.0,
            250_000.0,
            500_000.0,
            750_000.0,
            1_000_000.0,
            1_500_000.0,
            2_500_000.0,
            5_000_000.0,
            10_000_000.0,
            30_000_000.0,
            60_000_000.0
        ]
    )
    .unwrap();
}

pub fn endpoint() -> HttpResponse {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();

    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer.clone()).unwrap();

    // Create response and set content type
    HttpResponse::Ok().content_type("text/plain").body(body)
}
