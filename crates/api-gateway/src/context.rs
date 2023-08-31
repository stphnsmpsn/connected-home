use async_trait::async_trait;
use std::collections::HashMap;

use common::{
    context::ConnectedHomeServiceContext,
    error::{ConnectedHomeError, ConnectedHomeResult},
    util::{channel::Channel, cli::Args},
};
use slog::{o, Drain, Logger};
use tokio::{
    sync::{
        broadcast::{Receiver, Sender},
        RwLock,
    },
    task::Id,
};

use crate::{config::Config, metrics::Metrics};

pub struct Context {
    pub config: Config,
    pub metrics: Metrics,
    pub logger: Logger,
    shutdown: Channel<()>,
    task_map: RwLock<HashMap<Id, String>>,
}

impl Context {
    pub async fn from_args(args: Args<Config>) -> ConnectedHomeResult<Context> {
        let drain = slog_async::Async::new(slog_envlogger::new(
            slog_term::CompactFormat::new(slog_term::TermDecorator::new().build())
                .build()
                .fuse(),
        ))
        .build();

        let logger = Logger::root(drain.fuse(), o!());
        let metrics = Metrics::new(args.config.metrics.prefix.as_str())?;

        Ok(Self {
            config: args.config,
            logger,
            metrics,
            shutdown: Channel::default(),
            task_map: RwLock::new(HashMap::default()),
        })
    }
}

#[async_trait]
impl ConnectedHomeServiceContext for Context {
    async fn store_task_name(&self, id: Id, name: String) -> Result<(), ConnectedHomeError> {
        self.task_map.write().await.insert(id, name);
        Ok(())
    }

    async fn retrieve_task_name(&self, id: Id) -> Option<String> {
        self.task_map.read().await.get(&id).cloned()
    }

    fn shutdown_rx(&self) -> Receiver<()> {
        self.shutdown.rx()
    }

    fn shutdown_tx(&self) -> Sender<()> {
        self.shutdown.tx()
    }

    fn logger(&self) -> Logger {
        self.logger.clone()
    }
}
