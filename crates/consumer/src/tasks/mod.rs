use crate::context::Context;
use common::{error::ConnectedHomeResult, mqtt::events::ConnectedHomeEvent};
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;

pub mod mqtt_consumer;

pub async fn handle_event(ctx: Arc<Context>, event: ConnectedHomeEvent) -> ConnectedHomeResult<()> {
    match event {
        ConnectedHomeEvent::Current(current_measurement) => {
            ctx.metrics
                .current
                .last_measurement
                .with_label_values(&[current_measurement.device_id.to_string().as_str()])
                .set(current_measurement.amps.to_f64().unwrap());
        } // _ => {},
    };

    Ok(())
}
