use std::{iter::once, net::SocketAddr, sync::Arc};

use axum::{routing::get, Extension, Router};
use axum_tracing_opentelemetry::opentelemetry_tracing_layer;
use common::error::ConnectedHomeResult;
use http::{
    header::{HeaderName, AUTHORIZATION},
    Response, StatusCode,
};
use hyper::Body;
use tower_http::{
    add_extension::AddExtensionLayer, compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveRequestHeadersLayer, validate_request::ValidateRequestHeaderLayer,
};

use crate::{context::Context, server::layer::authorization_extractor::PropagateAuthorizationLayer};

mod layer;
mod realm;

pub struct Server {}

impl Server {
    pub async fn serve(context: Arc<Context>, router: Router) -> ConnectedHomeResult<()> {
        let bind_addr: SocketAddr = format!(
            "{}:{}",
            context.config.server.listen_address, context.config.server.port
        )
        .parse()?;

        let app = Router::new()
            .route("/metrics", get(get_metrics))
            .merge(realm::user::router())
            .merge(router)
            .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
            .layer(opentelemetry_tracing_layer())
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(HeaderName::from_static("x-request-id")))
            .layer(ValidateRequestHeaderLayer::accept("application/json"))
            .layer(PropagateAuthorizationLayer::new())
            .layer(AddExtensionLayer::new(context));

        Ok(axum::Server::bind(&bind_addr).serve(app.into_make_service()).await?)
    }
}

async fn get_metrics(Extension(ctx): Extension<Arc<Context>>) -> Result<Response<Body>, StatusCode> {
    let metrics =
        common::metrics::get_metrics(ctx.metrics.registry.clone()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut response = Response::new(Body::from(metrics));
    *response.status_mut() = StatusCode::OK;
    Ok(response)
}
