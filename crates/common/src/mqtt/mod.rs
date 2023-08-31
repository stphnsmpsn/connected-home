pub mod events;

use crate::util::cli::from_file_or_const;
use rumqttc::MqttOptions;
use std::time::Duration;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    #[serde(deserialize_with = "from_file_or_const")]
    pub username: String,
    #[serde(deserialize_with = "from_file_or_const")]
    pub password: String,
    pub client_id: String,
}

impl MqttConfig {
    pub fn options(&self) -> MqttOptions {
        println!("{:?}", self);

        let mut mqttoptions = MqttOptions::new(self.client_id.as_str(), self.host.as_str(), self.port);

        mqttoptions.set_credentials(self.username.as_str(), self.password.as_str());
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        mqttoptions
    }
}
