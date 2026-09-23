use super::{EventCloudEventCodec, EventConsumer, EventSubscriberConfig};
use crate::event::EventEnvelope;
use crate::event::EventSelector;
use crate::messaging::{
    CloudEventSelector, CloudEventSubscriber, CloudEventSubscriberError, ConsumerGroup, Subscriber,
    SubscriberError, Subscription,
};

pub struct EventSubscriber<S>
where
    S: CloudEventSubscriber,
{
    cloud_event_subscriber: S,
    config: EventSubscriberConfig,
}

impl<S> EventSubscriber<S>
where
    S: CloudEventSubscriber,
{
    pub fn new(cloud_event_subscriber: S, config: EventSubscriberConfig) -> Self {
        Self {
            cloud_event_subscriber,
            config,
        }
    }

    fn selector(&self, selector: &EventSelector) -> Result<CloudEventSelector, SubscriberError> {
        Ok(CloudEventSelector::new().with_type(
            EventCloudEventCodec::encode_type(
                self.config.type_prefix.as_ref(),
                &selector.aggregate_type.into(),
                &selector.event_name.into(),
            )
            .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?,
        ))
    }
}

impl<S> Subscriber<EventEnvelope> for EventSubscriber<S>
where
    S: CloudEventSubscriber,
{
    type Consumer = EventConsumer<S::Consumer>;
    type Selector = EventSelector;

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
        let group = consumer_group;
        let cloud_event_consumer = self
            .cloud_event_subscriber
            .subscribe(group, cloud_subscription)
            .await
            .map_err(|source| match source {
                CloudEventSubscriberError::InvalidSubscription => {
                    SubscriberError::InvalidSubscription
                }
                other => SubscriberError::Subscribe(Box::new(other)),
            })?;
        Ok(EventConsumer::new(
            cloud_event_consumer,
            self.config.type_prefix.clone(),
        ))
    }
}
