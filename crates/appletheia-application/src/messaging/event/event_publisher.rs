use super::EventPublisherConfig;
use crate::event::EventEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{CloudEventPublisher, PublishResult, Publisher, PublisherError};

pub struct EventPublisher<P> {
    publisher: P,
    config: EventPublisherConfig,
}

impl<P> EventPublisher<P> {
    pub fn new(publisher: P, config: EventPublisherConfig) -> Self {
        Self { publisher, config }
    }
}

impl<P: CloudEventPublisher> Publisher for EventPublisher<P> {
    type Message = EventEnvelope;

    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a Self::Message>,
    {
        let events = messages
            .into_iter()
            .map(|message| {
                message.try_to_cloud_event(&self.config.source, self.config.type_prefix.as_ref())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;
        self.publisher
            .publish(&events)
            .await
            .map_err(|source| PublisherError::Publish(Box::new(source)))
    }
}
