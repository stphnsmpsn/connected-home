use crate::{error::ConnectedHomeError, mqtt::events::energy::CurrentMeasurement};
use rumqttc::Publish;

pub mod energy;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ConnectedHomeEvent {
    Current(CurrentMeasurement),
}

impl TryFrom<&Publish> for ConnectedHomeEvent {
    type Error = ConnectedHomeError;

    fn try_from(publish: &Publish) -> Result<Self, Self::Error> {
        serde_json::from_slice::<ConnectedHomeEvent>(&publish.payload)
            .map_err(|_| ConnectedHomeError::ParseEvent(publish.payload.to_vec()))
    }
}
