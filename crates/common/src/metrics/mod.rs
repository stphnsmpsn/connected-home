use std::sync::Arc;

use prometheus::{Encoder, Registry, TextEncoder};

use crate::error::ConnectedHomeResult;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct MetricsConfig {
    pub prefix: String,
}

pub fn get_metrics(metrics_registry: Arc<Registry>) -> ConnectedHomeResult<String> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = metrics_registry.gather();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}
