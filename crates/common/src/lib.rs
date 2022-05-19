use hyper::{Body, Response};
use std::sync::{Arc, Mutex};
use warp::{http::StatusCode, Filter};

pub async fn ready_handler(ready: Arc<Mutex<bool>>) -> Result<impl warp::Reply, warp::Rejection> {
    let ready = ready.lock().unwrap();

    if *ready {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("X-Custom-Foo", "Bar")
            .body(Body::from(String::from("READY")))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .header("X-Custom-Foo", "Bar")
            .body(Body::from(String::from("NOT READY")))
            .unwrap())
    }
}

async fn healthy_handler(healthy: Arc<Mutex<bool>>) -> Result<impl warp::Reply, warp::Rejection> {
    let healthy = healthy.lock().unwrap();

    if *healthy {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("X-Custom-Foo", "Bar")
            .body(Body::from(String::from("HEALTHY")))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::SERVICE_UNAVAILABLE)
            .header("X-Custom-Foo", "Bar")
            .body(Body::from(String::from("NOT HEALTHY")))
            .unwrap())
    }
}

pub fn make_ready_filter(
    path: String,
    arg: Arc<Mutex<bool>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path)
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&arg)))
        .and_then(ready_handler)
}

pub fn make_healthy_filter(
    path: String,
    arg: Arc<Mutex<bool>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path)
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&arg)))
        .and_then(healthy_handler)
}
