use crate::api::{api, ServiceContext};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::http::{HeaderMap, HeaderValue, StatusCode};
use warp::{Filter, Rejection, Reply};

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

mod api;

#[derive(Serialize, Deserialize)]
enum Status {
    Ok,
}

#[tokio::main]
async fn main() {
    let opentelemetry = tracing_opentelemetry::layer()
        .with_tracer(
            opentelemetry_jaeger::new_agent_pipeline()
                .with_service_name("api-gateway")
                .with_endpoint(SocketAddr::from(([127, 0, 0, 1], 6831)))
                .install_simple()
                .unwrap(),
        )
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let stdout = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .with(stdout)
        .try_init()
        .unwrap();

    {
        let root = tracing::span!(tracing::Level::INFO, "app_start", work_units = 2);
        let _enter = root.enter();

        tracing::warn!("About to exit!");
        tracing::trace!("status: {}", true);
    } // Once this scope is closed, all spans inside are closed as well

    let service_context = Arc::new(ServiceContext::new());

    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("*"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("*"),
    );

    let api = warp::path("api")
        .and(warp::method())
        .and(warp::path::param())
        .and(warp::body::bytes())
        .and(warp::header::optional::<String>("authorization"))
        .and(with_context(service_context))
        .and_then(api)
        .with(warp::reply::with::headers(headers.clone()));

    let ready = warp::path("ready")
        .and(warp::get())
        .map(|| warp::reply::json(&Status::Ok))
        .with(warp::reply::with::headers(headers.clone()));

    let healthy = warp::path("healthy")
        .and(warp::get())
        .map(|| warp::reply::json(&Status::Ok))
        .with(warp::reply::with::headers(headers.clone()));

    let options = warp::options()
        .map(|| warp::reply::json(&Status::Ok))
        .with(warp::reply::with::headers(headers.clone()));

    let routes = options
        .or(healthy)
        .or(ready)
        .or(api)
        .recover(handle_rejection);

    // we listen on all interfaces because we will be inside of a container
    // and we do not know what IP we will be assigned
    warp::serve(routes).run(([0, 0, 0, 0], 9082)).await;
}

// todo: beef up this handler
async fn handle_rejection(_: Rejection) -> Result<impl Reply, Infallible> {
    let empty: Vec<u8> = Vec::new();
    Ok(warp::reply::with_status(empty, StatusCode::BAD_REQUEST))
}

fn with_context(
    context: Arc<ServiceContext>,
) -> impl Filter<Extract = (Arc<ServiceContext>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || context.clone())
}
