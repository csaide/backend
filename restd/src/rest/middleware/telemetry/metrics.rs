// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use prometheus;

lazy_static! {
    pub static ref REQUEST_COUNTER: prometheus::IntCounterVec = register_int_counter_vec!(
        "restd_rest_requests",
        "Number of requests handled by route/method/status.",
        &["route", "method", "status"]
    )
    .unwrap();

    pub static ref LATENCY_HISTOGRAM: prometheus::HistogramVec = register_histogram_vec!(
        "restd_rest_latency_ms",
        "Latency microsecond resolution buckets of requests handled by route/method/status.",
        &["route", "method", "status"],
        vec![
            1.0,      // 1ms
            250.0,    // 250ms
            500.0,    // 500ms
            750.0,    // 750ms
            1_000.0,  // 1s
            1_500.0,  // 1.5s
            2_500.0,  // 2.5s
            5_000.0,  // 5s
            10_000.0, // 10s
            30_000.0, // 30s
            60_000.0  // 60s
        ]
    )
    .unwrap();
}
