use hyper::header::ToStrError;
use hyper::{Body, Request};
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::{
    task::{Context, Poll},
    time::Instant,
};
use tonic::body::BoxBody;
use tonic::codegen::http::header::HeaderName;
use tonic::codegen::http::HeaderValue;
use tonic::metadata::{AsciiMetadataKey, AsciiMetadataValue, MetadataValue};
use tower::{Layer, Service};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use types::jwt::Jwt;

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Default)]
pub struct SendTracingContext {
    jwt: Option<Jwt>,
}

impl SendTracingContext {
    pub fn with_jwt(jwt: Jwt) -> Self {
        Self { jwt: Some(jwt) }
    }
}

impl tonic::service::Interceptor for SendTracingContext {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let span = tracing::Span::current();
        let context = span.context();
        let propagator = TraceContextPropagator::new();
        let mut context_map = HashMap::new();
        propagator.inject_context(&context, &mut context_map);

        let meta = request.metadata_mut();

        for (k, v) in context_map.into_iter() {
            let metadata_key =
                AsciiMetadataKey::from_bytes(HeaderName::try_from(k).unwrap().as_str().as_bytes())
                    .unwrap();

            let metadata_value =
                AsciiMetadataValue::try_from(HeaderValue::try_from(v).unwrap().as_bytes()).unwrap();

            meta.insert(metadata_key, metadata_value);
        }

        if let Some(jwt) = &self.jwt {
            request.metadata_mut().insert(
                "authorization",
                MetadataValue::try_from(jwt.to_string().as_str()).unwrap(),
            );
        }

        Ok(request)
    }
}

#[derive(Clone)]
pub struct RestoreTracingContextLayer {}

impl<S> Layer<S> for RestoreTracingContextLayer {
    type Service = Tracing<S>;

    fn layer(&self, service: S) -> Self::Service {
        Tracing { inner: service }
    }
}

#[derive(Clone)]
pub struct Tracing<S> {
    inner: S,
}

#[derive(Clone)]
pub struct RequestContext {
    pub tracing_context: opentelemetry::Context,
}

impl TryFrom<&hyper::Request<Body>> for RequestContext {
    type Error = ToStrError;

    fn try_from(req: &Request<Body>) -> Result<Self, Self::Error> {
        let parent_key = "traceparent".to_string();
        let state_key = "tracestate".to_string();

        let trace_parent = req.headers().get(&parent_key);
        let trace_state = req.headers().get(&state_key);

        let mut fields: HashMap<String, String> = HashMap::new();

        if let Some(trace_parent) = trace_parent {
            fields.insert(parent_key, trace_parent.to_str()?.to_string());
        }

        if let Some(trace_state) = trace_state {
            fields.insert(state_key, trace_state.to_str()?.to_string());
        }

        Ok(Self {
            tracing_context: TraceContextPropagator::new().extract(&fields),
        })
    }
}

impl<S> Service<hyper::Request<Body>> for Tracing<S>
where
    S: Service<hyper::Request<Body>, Response = hyper::Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<Body>) -> Self::Future {
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        match RequestContext::try_from(&req) {
            Ok(ctx) => {
                req.extensions_mut().insert(ctx.clone());
            }
            _ => {
                tracing::warn!("received request without valid tracing context");
            }
        };

        Box::pin(async move {
            let started = Instant::now();
            let response: Self::Response = inner.call(req).await?;
            let _elapsed = started.elapsed();
            Ok(response)
        })
    }
}
