use axum::http::Request;
use common::auth::jwt::Jwt;
use hyper::Body;
use std::{
    convert::{Infallible, TryFrom},
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct PropagateAuthorization<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for PropagateAuthorization<S>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let maybe_jwt = req
            .headers()
            .get("Authorization")
            .and_then(|header_value| Jwt::try_from(header_value).ok());

        req.extensions_mut().insert(maybe_jwt);

        self.inner.call(req)
    }
}

#[derive(Clone)]
pub struct PropagateAuthorizationLayer;

impl PropagateAuthorizationLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for PropagateAuthorizationLayer {
    type Service = PropagateAuthorization<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PropagateAuthorization { inner }
    }
}
