#[macro_use]
extern crate log;

use crate::api::api;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::http::{HeaderMap, HeaderValue, StatusCode};
use warp::{Filter, Rejection, Reply};

mod api;

#[derive(Serialize, Deserialize)]
enum Status {
    Ok,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("API Gateway Starting Up...");

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
    warp::serve(routes).run(([0, 0, 0, 0], 8082)).await;
}

// todo: beef up this handler
async fn handle_rejection(_: Rejection) -> Result<impl Reply, Infallible> {
    let empty: Vec<u8> = Vec::new();
    Ok(warp::reply::with_status(empty, StatusCode::BAD_REQUEST))
}
