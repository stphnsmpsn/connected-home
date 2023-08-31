use slog::{error, info, warn};
use std::{fmt::Display, future::Future, sync::Arc, time::Duration};
use tokio::{
    signal::unix::{signal, SignalKind},
    task::{Id, JoinError, JoinSet},
};

use crate::{context::ConnectedHomeServiceContext, error::ConnectedHomeError};

pub type ConnectedHomeTaskResult<E> = Result<(), E>;

pub enum ShutdownType {
    /// Some tasks may not be safe to abruptly abort and could require more complex shutdown logic.
    /// For these tasks, they should have a tokio::sync::broadcast:Receiver they are listening for
    /// shutdown signals on so that they can gracefully exit on their own.
    Integrated,
    /// For tasks that are safe to abruptly abort, they can be launched with this type. This will
    /// cause the task to be launched with a tokio::select! that will listen for a shutdown signal
    /// and will exit if it receives one.
    Manual,
}

pub async fn launch_task<C, E>(
    task_name: String,
    task: impl Future<Output = ConnectedHomeTaskResult<E>>,
    context: Arc<C>,
    shutdown_type: ShutdownType,
) -> Result<(), E>
where
    C: ConnectedHomeServiceContext,
    E: From<ConnectedHomeError>,
{
    let task_id = tokio::task::id();
    context.store_task_name(task_id, task_name.clone()).await?;

    info!(
        context.logger(),
        "Starting Task...";
        "task_name" => task_name,
    );

    match shutdown_type {
        ShutdownType::Integrated => task.await,
        ShutdownType::Manual => {
            let mut shutdown_rx = context.shutdown_rx();
            tokio::select! {
                _ = shutdown_rx.recv() => Ok(()),
                res = task => res,
            }
        }
    }
}

pub async fn await_signal<C, E>(context: Arc<C>) -> ConnectedHomeTaskResult<E>
where
    C: ConnectedHomeServiceContext,
{
    let signal_kind = tokio::select! {
        kind = wait_for_signal(SignalKind::terminate()) => kind,
        kind = wait_for_signal(SignalKind::interrupt()) => kind,
        kind = wait_for_signal(SignalKind::hangup()) => kind,
    };

    info!(context.logger(), "Received shutdown signal..."; "signal_kind" => ?signal_kind);
    info!(context.logger(), "Graceful shutdown initiated");

    Ok(())
}

async fn wait_for_signal(kind: SignalKind) -> SignalKind {
    signal(kind).expect("shutdown_listener").recv().await;
    kind
}

pub async fn wait_for_tasks<C, E>(set: &mut JoinSet<ConnectedHomeTaskResult<E>>, context: Arc<C>)
where
    C: ConnectedHomeServiceContext,
    E: 'static + Display,
{
    let first_exited_task = set.join_next_with_id().await;
    let drained_successfully = tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(context.graceful_shutdown_seconds())) => {
            set.abort_all();
            false
        },
        _ = drain_join_set(set, context.clone(), first_exited_task) => true,
    };

    if !drained_successfully {
        drain_join_set(set, context.clone(), None).await;
    }
}

async fn drain_join_set<C, E>(
    set: &mut JoinSet<ConnectedHomeTaskResult<E>>,
    ctx: Arc<C>,
    first_exited_task: Option<Result<(Id, ConnectedHomeTaskResult<E>), JoinError>>,
) where
    C: ConnectedHomeServiceContext,
    E: 'static + Display,
{
    if let Some(result) = first_exited_task {
        log_completed_task(result, ctx.clone()).await;
    }
    broadcast_shutdown(set, ctx.clone()).await;
    while let Some(result) = set.join_next_with_id().await {
        log_completed_task(result, ctx.clone()).await;
    }
}

async fn log_completed_task<C, E>(result: Result<(Id, ConnectedHomeTaskResult<E>), JoinError>, context: Arc<C>)
where
    C: ConnectedHomeServiceContext,
    E: Display,
{
    match result {
        Ok((id, Ok(()))) => {
            info!(
                context.logger(),
                "Task shut down cleanly";
                "task_name" => context.retrieve_task_name(id).await
            );
        }
        Ok((id, Err(err))) => {
            warn!(
                context.logger(),
                "Task shut down with errors";
                "task_name" => context.retrieve_task_name(id).await,
                "error" => %err,
            )
        }
        Err(err) => {
            warn!(
                context.logger(),
                "Task died";
                "task_name" => context.retrieve_task_name(err.id()).await,
                "error" => %err,
            );
        }
    }
}

async fn broadcast_shutdown<C, E>(set: &mut JoinSet<ConnectedHomeTaskResult<E>>, context: Arc<C>)
where
    C: ConnectedHomeServiceContext,
    E: 'static + Display,
{
    if let Err(err) = context.shutdown_tx().send(()) {
        error!(context.logger(), "Error sending graceful shutdown signal, performing dirty shutdown."; "error" => ?err);
        set.shutdown().await;
    }
}
