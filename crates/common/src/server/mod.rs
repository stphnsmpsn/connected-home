#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ServerConfig {
    pub port: u16,
    pub listen_address: String,
}
