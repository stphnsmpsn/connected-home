#[macro_use]
extern crate diesel;

use common::{make_healthy_filter, make_ready_filter};
use diesel::{Connection, PgConnection};
use grpc::user::user_service_server::UserServiceServer;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use user::handlers::MyUserService;
use warp::Filter;

mod error;
mod schema;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opentelemetry = tracing_opentelemetry::layer()
        .with_tracer(
            opentelemetry_jaeger::new_agent_pipeline()
                .with_service_name("user-service")
                .with_endpoint("jaeger:6831")
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

    let ready_flag = Arc::new(Mutex::new(false));

    let ready = make_ready_filter(String::from("ready"), ready_flag.clone());
    let healthy = make_healthy_filter(String::from("healthy"), ready_flag.clone());

    let routes = healthy.or(ready);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    let (health_checks, abort_handle) =
        futures_util::future::abortable(tokio::spawn(warp::serve(routes).run(addr)));

    // todo: manage config & secrets
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::debug!("Database URL: {}", database_url);

    let connection = Arc::new(Mutex::new(
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
    ));

    let addr = ([0, 0, 0, 0], 8083).into();
    let user_service = MyUserService::new(connection.clone());

    {
        let mut r = ready_flag.lock().unwrap();
        *r = true;
    }

    let layer = tower::ServiceBuilder::new()
        .layer(grpc::RestoreTracingContextLayer {})
        .into_inner();

    Server::builder()
        .layer(layer)
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;
    abort_handle.abort();
    let _res = health_checks.await;

    Ok(())
}
