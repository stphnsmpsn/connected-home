use crate::mqtt::events::energy::CurrentMeasurement;

pub mod energy;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ConnectedHomeEvent {
    Current(CurrentMeasurement),
}
