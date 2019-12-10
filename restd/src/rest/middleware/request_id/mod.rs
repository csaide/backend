// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{FromState, State};
use hyper::{http::HeaderValue, HeaderMap};
use std::io;
use uuid::Uuid;

#[derive(StateData)]
pub struct RequestID {
    id: String,
}

impl RequestID {
    pub fn as_str(&self) -> &str {
        self.id.as_str()
    }
}

pub struct Handler {}

impl NewMiddleware for Handler {
    type Instance = Self;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(Handler {})
    }
}

impl Middleware for Handler {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        let id = Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();

        let headers = HeaderMap::borrow_mut_from(&mut state);
        headers.insert(
            "x-request-id",
            HeaderValue::from_str(&id).expect("request id header value......."),
        );

        state.put(RequestID { id: id });

        Box::new(chain(state))
    }
}
