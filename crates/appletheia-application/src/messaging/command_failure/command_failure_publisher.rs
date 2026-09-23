use super::CommandFailurePublisherConfig;
use crate::command::CommandFailureEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{CloudEventPublisher, PublishResult, Publisher, PublisherError};

pub struct CommandFailurePublisher<P>
where
    P: CloudEventPublisher,
{
    publisher: P,
    config: CommandFailurePublisherConfig,
}

impl<P> CommandFailurePublisher<P>
where
    P: CloudEventPublisher,
{
    pub fn new(publisher: P, config: CommandFailurePublisherConfig) -> Self {
        Self { publisher, config }
    }
}

impl<P> Publisher for CommandFailurePublisher<P>
where
    P: CloudEventPublisher,
{
    type Message = CommandFailureEnvelope;

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
