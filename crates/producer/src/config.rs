use common::{metrics::MetricsConfig, mqtt::MqttConfig, tracing::config::TracingConfig};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
    pub mqtt: MqttConfig,
}
