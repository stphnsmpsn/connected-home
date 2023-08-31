#[macro_use]
extern crate slog;

use crate::{config::Config, context::Context, tasks::current_monitor::monitor_current};
use common::{
    error::{ConnectedHomeError, ConnectedHomeResult},
    tracing::init_tracing,
    util::{
        cli::{Args, Parser},
        task::{await_signal, launch_task, wait_for_tasks, ShutdownType},
    },
};
use rumqttc::AsyncClient;
use std::sync::Arc;
use tokio::task::JoinSet;

mod config;
mod context;
mod metrics;
mod tasks;

#[tokio::main]
async fn main() -> ConnectedHomeResult<()> {
    let args: Args<Config> = Args::parse();
    let context = Arc::new(Context::from_args(args).await?);

    init_tracing(context.config.tracing.clone());

    info!(
        context.logger,
        "Energy Monitor Started";
        "version" => env!("CARGO_PKG_VERSION")
    );

    let mut set = JoinSet::new();

    let (mqtt_client, mut mqtt_event_loop) = AsyncClient::new(context.config.mqtt.options(), 10);

    set.spawn(launch_task(
        "Sampler / Producer".to_string(),
        monitor_current(mqtt_client),
        context.clone(),
        ShutdownType::Manual,
    ));

    set.spawn(launch_task(
        "MQTT Event Loop".to_string(),
        async move {
            loop {
                mqtt_event_loop.poll().await.map_err(|_| ConnectedHomeError::Mqtt)?;
            }
        },
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
