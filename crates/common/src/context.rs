use async_trait::async_trait;
use slog::Logger;
use tokio::{
    sync::broadcast::{Receiver, Sender},
    task::Id,
};

use crate::error::ConnectedHomeError;

#[async_trait]
pub trait ConnectedHomeServiceContext {
    async fn store_task_name(&self, id: Id, name: String) -> Result<(), ConnectedHomeError>;
    async fn retrieve_task_name(&self, id: Id) -> Option<String>;
    fn graceful_shutdown_seconds(&self) -> u64 {
        3
    }
    fn shutdown_rx(&self) -> Receiver<()>;
    fn shutdown_tx(&self) -> Sender<()>;
    fn logger(&self) -> Logger;
}
