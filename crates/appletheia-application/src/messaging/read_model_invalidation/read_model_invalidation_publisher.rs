use super::ReadModelInvalidationPublisherConfig;
use crate::messaging::PublishableMessage;
use crate::messaging::{CloudEventPublisher, PublishResult, Publisher, PublisherError};
use crate::read_model::ReadModelInvalidationEnvelope;

pub struct ReadModelInvalidationPublisher<P>
where
    P: CloudEventPublisher,
{
    cloud_event_publisher: P,
    config: ReadModelInvalidationPublisherConfig,
}

impl<P> ReadModelInvalidationPublisher<P>
where
    P: CloudEventPublisher,
{
    pub fn new(cloud_event_publisher: P, config: ReadModelInvalidationPublisherConfig) -> Self {
        Self {
            cloud_event_publisher,
            config,
        }
    }
}

impl<P> Publisher for ReadModelInvalidationPublisher<P>
where
    P: CloudEventPublisher,
{
    type Message = ReadModelInvalidationEnvelope;

    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a Self::Message>,
    {
        let events = messages
            .into_iter()
            .map(|message| {
                message.try_to_cloud_event(
                    &self.config.source,
                    self.config.cloud_event_type_prefix.as_ref(),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;
        self.cloud_event_publisher
            .publish(&events)
            .await
            .map_err(|source| PublisherError::Publish(Box::new(source)))
    }
}
