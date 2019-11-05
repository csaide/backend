// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, http::header};
use futures::future::{err, ok, Either, FutureResult};
use futures::{Future, Poll};

#[derive(Clone)]
pub struct Authenticator {}

impl Authenticator {
    pub fn new() -> Authenticator {
        Authenticator {}
    }
}

impl<S, B> Transform<S> for Authenticator
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthenticatorMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticatorMiddleware { service })
    }
}

pub struct AuthenticatorMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticatorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        let mut token: &str = "missing";

        for auth_header in headers.get_all(header::HeaderName::from_static("authorization")) {
            let value = match auth_header.to_str() {
                Ok(value) => value,
                Err(_) => unimplemented!(),
            };

            let split: Vec<&str> = value.rsplit(' ').collect();

            if split.len() != 2 || split[1].to_lowercase() != "bearer" {
                return Box::new(Either::A(err(actix_web::Error::from(
                    super::model::error::unauthorized(),
                )
                .into())));
            } else {
                token = split[0];
                break;
            }
        }
        if token != "woot" {
            panic!(format!("second token doesn't equal woot got: '{}'", token))
        } else {
        }
        //

        Box::new(Either::B(
            self.service.call(req).and_then(move |res| Ok(res)),
        ))
    }
}
