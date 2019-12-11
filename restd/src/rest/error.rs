// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
    #[serde(skip_serializing)]
    pub status: StatusCode,
    pub msg: String,
    pub retryable: bool,
}

impl IntoResponse for HttpError {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            self.status,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialize http error"),
        )
    }
}
