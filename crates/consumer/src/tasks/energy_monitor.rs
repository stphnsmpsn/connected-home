use crate::context::Context;
use common::{
    error::{ConnectedHomeError, ConnectedHomeResult},
    mqtt::events::energy::CurrentMeasurement,
};
use rumqttc::{AsyncClient, QoS};
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;

pub async fn monitor_energy(ctx: Arc<Context>) -> ConnectedHomeResult<()> {
    let (mqtt_client, mut mqtt_event_loop) = AsyncClient::new(ctx.config.mqtt.options(), 10);

    mqtt_client.subscribe("energy", QoS::AtMostOnce).await.unwrap();
    loop {
        match mqtt_event_loop.poll().await.map_err(|_| ConnectedHomeError::Mqtt)? {
            rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) => {
                let current_measurement: CurrentMeasurement = serde_json::from_slice(&p.payload).unwrap();
                ctx.metrics
                    .current
                    .last_measurement
                    .with_label_values(&[current_measurement.device_id.to_string().as_str()])
                    .set(current_measurement.amps.to_f64().unwrap());
            }
            _ => {}
        }
    }
}
