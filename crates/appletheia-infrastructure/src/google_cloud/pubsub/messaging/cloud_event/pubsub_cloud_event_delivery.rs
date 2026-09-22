use appletheia_application::{CloudEvent, CloudEventDelivery, CloudEventDeliveryError};
use google_cloud_pubsub::subscriber::handler::Handler;

pub struct PubsubCloudEventDelivery {
    handler: Option<Handler>,
    message: CloudEvent,
}

impl PubsubCloudEventDelivery {
    pub(crate) fn new(handler: Handler, message: CloudEvent) -> Self {
        Self {
            handler: Some(handler),
            message,
        }
    }
}

impl CloudEventDelivery for PubsubCloudEventDelivery {
    fn message(&self) -> &CloudEvent {
        &self.message
    }

    async fn ack(&mut self) -> Result<(), CloudEventDeliveryError> {
        if let Some(handler) = self.handler.take() {
            handler.ack();
        }
        Ok(())
    }

    async fn nack(&mut self) -> Result<(), CloudEventDeliveryError> {
        if let Some(handler) = self.handler.take() {
            drop(handler);
        }
        Ok(())
    }
}
