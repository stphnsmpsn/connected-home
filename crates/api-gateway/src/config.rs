use common::{metrics::MetricsConfig, server::ServerConfig, tracing::config::TracingConfig};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
    pub remote: RemoteConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct RemoteConfig {
    pub user_service: String, // TODO: use Url
}
