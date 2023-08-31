#[macro_use]
extern crate slog;

use std::{env, sync::Arc};

use axum::Router;
use tokio::task::JoinSet;

use common::{
    error::ConnectedHomeResult,
    tracing::init_tracing,
    util::{
        cli::{Args, Parser},
        task::{await_signal, launch_task, wait_for_tasks, ShutdownType},
    },
};

use crate::{config::Config, context::Context};

mod config;
mod context;
mod error;
mod metrics;
mod repo;
mod server;
mod user;

#[tokio::main]
async fn main() -> ConnectedHomeResult<()> {
    let args: Args<Config> = Args::parse();
    let context = Arc::new(Context::from_args(args).await?);

    init_tracing(context.config.tracing.clone());

    info!(
        context.logger,
        "User Service Started";
        "version" => env!("CARGO_PKG_VERSION")
    );

    sqlx::migrate!("./migrations")
        .run(&mut context.get_db_conn().await?)
        .await?;

    let mut set = JoinSet::new();

    set.spawn(launch_task(
        "HTTP Server".to_string(),
        server::http::HttpServer::serve(context.clone(), Router::new()),
        context.clone(),
        ShutdownType::Manual,
    ));

    set.spawn(launch_task(
        "GRPC Server".to_string(),
        server::grpc::GrpcServer::serve(context.clone()),
        context.clone(),
        ShutdownType::Manual,
    ));

    set.spawn(launch_task(
        "Signal Catcher".to_string(),
        await_signal(context.clone()),
        context.clone(),
        ShutdownType::Manual,
    ));

    wait_for_tasks(&mut set, context.clone()).await;

    Ok(())
}
