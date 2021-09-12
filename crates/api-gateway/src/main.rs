#![deny(warnings)]
use api_gateway::{user_connected, Users, INDEX_HTML};
use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
enum Status {
    Ok,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let chat = warp::path("chat")
        .and(warp::get())
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| ws.on_upgrade(move |socket| user_connected(socket, users)));

    let home = warp::path("home")
        .and(warp::get())
        .map(|| warp::reply::html(INDEX_HTML));

    let ready = warp::path("ready")
        .and(warp::get())
        .map(|| warp::reply::json(&Status::Ok));

    let healthy = warp::path("healthy")
        .and(warp::get())
        .map(|| warp::reply::json(&Status::Ok));

    let routes = home.or(chat).or(healthy).or(ready);

    // we listen on all interfaces because we will be inside of a container
    // and we do not know what IP we will be assigned
    warp::serve(routes).run(([0, 0, 0, 0], 8082)).await;
}
