use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CurrentMeasurement {
    pub device_id: Uuid,
    pub amps: Decimal,
    pub timestamp: DateTime<Utc>,
}
