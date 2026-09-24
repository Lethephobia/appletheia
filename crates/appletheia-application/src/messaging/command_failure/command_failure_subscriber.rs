use super::{CommandFailureConsumer, CommandFailureSubscriberConfig};
use crate::command::CommandFailureEnvelope;
use crate::messaging::{
    CloudEventSelector, CloudEventSubscriber, CloudEventSubscriberError, ConsumerGroup, Subscriber,
    SubscriberError, Subscription,
};
use crate::saga::SagaName;

pub struct CommandFailureSubscriber<S>
where
    S: CloudEventSubscriber,
{
    cloud_event_subscriber: S,
    config: CommandFailureSubscriberConfig,
}

impl<S> CommandFailureSubscriber<S>
where
    S: CloudEventSubscriber,
{
    pub fn new(cloud_event_subscriber: S, config: CommandFailureSubscriberConfig) -> Self {
        Self {
            cloud_event_subscriber,
            config,
        }
    }

    fn selector(&self, selector: &SagaName) -> Result<CloudEventSelector, SubscriberError> {
        Ok(CloudEventSelector::new().with_extension(
            "saganame"
                .parse()
                .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?,
            selector
                .value()
                .parse()
                .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?,
        ))
    }
}

impl<S> Subscriber<CommandFailureEnvelope> for CommandFailureSubscriber<S>
where
    S: CloudEventSubscriber,
{
    type Consumer = CommandFailureConsumer<S::Consumer>;
    type Selector = SagaName;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let selectors;
        let cloud_subscription = match subscription {
            Subscription::All => Subscription::All,
            Subscription::AnyOf([]) => return Err(SubscriberError::InvalidSubscription),
            Subscription::AnyOf(values) => {
                selectors = values
                    .iter()
                    .map(|selector| self.selector(selector))
                    .collect::<Result<Vec<_>, SubscriberError>>()?;

                Subscription::AnyOf(&selectors)
            }
            Subscription::One(value) => {
                selectors = vec![self.selector(value)?];

                Subscription::One(&selectors[0])
            }
        };
        let cloud_event_consumer = self
            .cloud_event_subscriber
            .subscribe(consumer_group, cloud_subscription)
            .await
            .map_err(|source| match source {
                CloudEventSubscriberError::InvalidSubscription => {
                    SubscriberError::InvalidSubscription
                }
                other => SubscriberError::Subscribe(Box::new(other)),
            })?;
        Ok(CommandFailureConsumer::new(
            cloud_event_consumer,
            self.config.cloud_event_type_prefix.clone(),
        ))
    }
}
