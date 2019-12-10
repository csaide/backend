// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use gotham::handler::IntoResponse;
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    #[serde(skip_serializing)]
    status: StatusCode,
    alive: bool,
}

impl IntoResponse for Health {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            self.status,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self).expect("serialize health"),
        )
    }
}

pub fn endpoint(state: State) -> (State, Health) {
    (
        state,
        Health {
            status: StatusCode::OK,
            alive: true,
        },
    )
}
