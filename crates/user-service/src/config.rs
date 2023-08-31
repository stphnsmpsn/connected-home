use common::{metrics::MetricsConfig, repo::PostgresConfig, server::ServerConfig, tracing::config::TracingConfig};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Config {
    pub http_server: ServerConfig,
    pub grpc_server: ServerConfig,
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
    pub remote: RemoteConfig,
    pub postgres: PostgresConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct RemoteConfig {}
