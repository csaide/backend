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
        "restd_rest_latency",
        "Latency buckets of requests handled by route/method/status.",
        &["route", "method", "status"],
        vec![
            0.001,  // 1ms
            0.250,  // 250ms
            0.500,  // 500ms
            0.750,  // 750ms
            1.000,  // 1s
            1.500,  // 1.5s
            2.500,  // 2.5s
            5.000,  // 5s
            10.000, // 10s
            30.000, // 30s
            60.000  // 60s
        ]
    )
    .unwrap();

    pub static ref RESPONSE_SIZE_HISTOGRAM: prometheus::HistogramVec = register_histogram_vec!(
        "restd_rest_response_size",
        "Response size buckets of requests handled by route/method/status.",
        &["route", "method", "status"],
        vec![
            256.0,     // 256 B
            512.0,     // 512 B
            1024.0,    // 1 KiB
            262144.0,  // 256 KiB
            524288.0,  // 512 KiB
            1048576.0, // 1 MiB
            2097152.0, // 2 MiB
            5242880.0, // 5 MiB
        ]
    )
    .unwrap();
}
