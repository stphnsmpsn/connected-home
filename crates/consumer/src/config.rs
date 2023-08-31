use common::{metrics::MetricsConfig, mqtt::MqttConfig, server::ServerConfig, tracing::config::TracingConfig};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
    pub mqtt: MqttConfig,
}
