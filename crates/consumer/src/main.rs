#[macro_use]
extern crate slog;

use crate::{config::Config, context::Context, tasks::mqtt_consumer::MqttConsumer};
use axum::Router;
use common::{
    error::ConnectedHomeResult,
    tracing::init_tracing,
    util::{
        cli::{Args, Parser},
        task::{await_signal, launch_task, wait_for_tasks, ShutdownType},
    },
};
use std::sync::Arc;
use tokio::task::JoinSet;

mod config;
mod context;
mod metrics;
mod server;
mod tasks;

#[tokio::main]
async fn main() -> ConnectedHomeResult<()> {
    let args: Args<Config> = Args::parse();
    let context = Arc::new(Context::from_args(args).await?);

    init_tracing(context.config.tracing.clone());

    info!(
        context.logger,
        "MQTT Consumer";
        "version" => env!("CARGO_PKG_VERSION")
    );

    let mut set = JoinSet::new();

    set.spawn(launch_task(
        "MQTT Consumer".to_string(),
        {
            let mut consumer = MqttConsumer::new(context.clone());
            async move { consumer.start().await }
        },
        context.clone(),
        ShutdownType::Manual,
    ));

    set.spawn(launch_task(
        "HTTP Server".to_string(),
        server::Server::serve(context.clone(), Router::new()),
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
