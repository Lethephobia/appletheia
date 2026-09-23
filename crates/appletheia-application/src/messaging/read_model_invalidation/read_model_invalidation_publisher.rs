use super::ReadModelInvalidationPublisherConfig;
use crate::messaging::PublishableMessage;
use crate::messaging::{CloudEventPublisher, PublishResult, Publisher, PublisherError};
use crate::read_model::ReadModelInvalidationEnvelope;

pub struct ReadModelInvalidationPublisher<P> {
    publisher: P,
    config: ReadModelInvalidationPublisherConfig,
}

impl<P> ReadModelInvalidationPublisher<P> {
    pub fn new(publisher: P, config: ReadModelInvalidationPublisherConfig) -> Self {
        Self { publisher, config }
    }
}

impl<P: CloudEventPublisher> Publisher for ReadModelInvalidationPublisher<P> {
    type Message = ReadModelInvalidationEnvelope;

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
