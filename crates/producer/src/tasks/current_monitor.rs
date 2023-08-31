use chrono::Utc;
use common::{
    error::{ConnectedHomeError, ConnectedHomeResult},
    mqtt::events::energy::CurrentMeasurement,
};
use rand::Rng;
use rumqttc::{AsyncClient, QoS};
use rust_decimal::Decimal;
use std::time::Duration;
use uuid::Uuid;

pub async fn monitor_current(mqtt_client: AsyncClient) -> ConnectedHomeResult<()> {
    let device_one = Uuid::new_v4();
    let device_two = Uuid::new_v4();
    loop {
        let amps = sample_current();
        publish_current_measurement(&mqtt_client, device_one, amps).await?;
        let amps = sample_current();
        publish_current_measurement(&mqtt_client, device_two, amps).await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}

fn sample_current() -> f64 {
    let mut rng = rand::thread_rng();
    let number: f64 = rng.gen_range(8.0..=9.0);
    number
}

async fn publish_current_measurement(mqtt_client: &AsyncClient, device_id: Uuid, amps: f64) -> ConnectedHomeResult<()> {
    let measurement = CurrentMeasurement {
        device_id,
        timestamp: Utc::now(),
        amps: Decimal::try_from(amps).unwrap(),
    };
    let serialized_measurement = serde_json::to_string(&measurement).unwrap();

    Ok(mqtt_client
        .publish("energy", QoS::AtLeastOnce, true, serialized_measurement)
        .await
        .map_err(|_| ConnectedHomeError::Mqtt)?)
}
