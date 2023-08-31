#[macro_use]
extern crate slog;

use axum::Router;
use std::sync::Arc;
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
mod metrics;
mod server;

#[tokio::main]
async fn main() -> ConnectedHomeResult<()> {
    let args: Args<Config> = Args::parse();
    let context = Arc::new(Context::from_args(args).await?);

    init_tracing(context.config.tracing.clone());

    info!(
        context.logger,
        "API Gateway Started";
        "version" => env!("CARGO_PKG_VERSION")
    );

    let mut set = JoinSet::new();

    set.spawn(launch_task(
        "HTTP Server".to_string(),
        server::Server::serve(context.clone(), build_router()),
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

#[allow(dead_code)]
fn build_router() -> Router {
    Router::new()
}
