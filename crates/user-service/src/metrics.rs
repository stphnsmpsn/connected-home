use common::error::ConnectedHomeResult;
use prometheus::Registry;
use std::sync::Arc;

pub struct Metrics {
    // wrapped in ARC b/c it is passed as shared state to GET /metrics
    pub registry: Arc<Registry>,
}

impl Metrics {
    pub fn new(prefix: &str) -> ConnectedHomeResult<Self> {
        let registry = Registry::new_custom(Some(prefix.to_string()), None)?;
        Ok(Self {
            registry: Arc::new(registry),
        })
    }
}
