use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::HeaderName;
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, FutureResult};
use futures::Poll;

use super::error;

pub struct Authenticator;

impl<S, B> Transform<S> for Authenticator
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
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
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let auth_header_name = HeaderName::from_static("authorization");
        let headers = req.headers();

        if !headers.contains_key(&auth_header_name) {
            return Either::B(ok(req.into_response(
                HttpResponse::from_error(error::forbidden().to_actix()).into_body(),
            )));
        }

        let auth_headers = headers.get_all(&auth_header_name);
        for header in auth_headers {
            if header == "Bearer woot" {
                return Either::A(self.service.call(req));
            }
        }

        Either::B(ok(req.into_response(
            HttpResponse::from_error(error::unauthorized().to_actix()).into_body(),
        )))
    }
}
