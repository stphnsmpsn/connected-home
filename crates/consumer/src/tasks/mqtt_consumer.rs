use crate::{context::Context, tasks::handle_event};
use common::{
    error::{ConnectedHomeError, ConnectedHomeResult},
    mqtt::events::ConnectedHomeEvent,
};
use rumqttc::AsyncClient;
use std::sync::Arc;

pub struct MqttConsumer {
    ctx: Arc<Context>,
    mqtt_client: AsyncClient,
    mqtt_event_loop: rumqttc::EventLoop,
}

impl MqttConsumer {
    pub fn new(ctx: Arc<Context>) -> Self {
        let (mqtt_client, mqtt_event_loop) = AsyncClient::new(ctx.config.mqtt.options(), 10);
        Self {
            ctx,
            mqtt_client,
            mqtt_event_loop,
        }
    }

    pub async fn start(&mut self) -> ConnectedHomeResult<()> {
        self.mqtt_client
            .subscribe_many(self.ctx.config.mqtt.sub_filters())
            .await
            .map_err(|_| ConnectedHomeError::Mqtt)?;

        loop {
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(p)) = self
                .mqtt_event_loop
                .poll()
                .await
                .map_err(|_| ConnectedHomeError::Mqtt)?
            {
                let event = ConnectedHomeEvent::try_from(&p)?;
                handle_event(self.ctx.clone(), event).await?;
            }
        }
    }
}
