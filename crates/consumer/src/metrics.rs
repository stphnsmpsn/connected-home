use common::error::ConnectedHomeResult;
use prometheus::{opts, GaugeVec, Registry};
use std::sync::Arc;

pub struct Metrics {
    // wrapped in ARC b/c it is passed as shared state to GET /metrics
    pub registry: Arc<Registry>,
    pub current: CurrentMetrics,
}

impl Metrics {
    pub fn new(prefix: &str) -> ConnectedHomeResult<Self> {
        let registry = Registry::new_custom(Some(prefix.to_string()), None)?;
        let current = CurrentMetrics::new(registry.clone())?;
        Ok(Self {
            registry: Arc::new(registry),
            current,
        })
    }
}

pub struct CurrentMetrics {
    pub last_measurement: GaugeVec,
}

impl CurrentMetrics {
    fn new(registry: Registry) -> ConnectedHomeResult<Self> {
        let last_measurement = GaugeVec::new(
            opts!("current_last_measurement", "Current Last Measurement"),
            &["device_id"],
        )?;
        registry.register(Box::new(last_measurement.clone()))?;
        Ok(Self { last_measurement })
    }
}
