use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use std::{convert::Infallible, net::SocketAddr};

struct State {
    _val: u64,
}

async fn ready_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    //let state = req.data::<State>().unwrap();
    //println!("State value: {}", state.0);
    Ok(Response::new(Body::from("OK")))
}

async fn healthy_handler(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    //let state = req.data::<State>().unwrap();
    //println!("State value: {}", state.0);
    Ok(Response::new(Body::from("OK")))
}

async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

fn router() -> Router<Body, Infallible> {
    Router::builder()
        .data(State { _val: 100 })
        .middleware(Middleware::pre(logger))
        .get("/ready", ready_handler)
        .get("/healthy", healthy_handler)
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router = router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    let server = Server::bind(&addr).serve(service);
    println!("Consumer is running on: {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
