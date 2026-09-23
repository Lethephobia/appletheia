use super::{EventConsumer, EventSubscriberConfig};
use crate::event::EventEnvelope;
use crate::event::EventSelector;
use crate::messaging::CloudEventType;
use crate::messaging::{
    CloudEventSelector, CloudEventSubscriber, CloudEventSubscriberError, ConsumerGroup, Subscriber,
    SubscriberError, Subscription,
};

pub struct EventSubscriber<S> {
    subscriber: S,
    config: EventSubscriberConfig,
}

impl<S> EventSubscriber<S> {
    pub fn new(subscriber: S, config: EventSubscriberConfig) -> Self {
        Self { subscriber, config }
    }

    fn selector(&self, selector: &EventSelector) -> Result<CloudEventSelector, SubscriberError> {
        Ok(CloudEventSelector::new().with_type(
            CloudEventType::with_prefix(
                self.config.type_prefix.as_ref(),
                &format!("{}.{}", selector.aggregate_type, selector.event_name),
            )
            .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?,
        ))
    }
}

impl<S: CloudEventSubscriber> Subscriber<EventEnvelope> for EventSubscriber<S> {
    type Consumer = EventConsumer<S::Consumer>;
    type Selector = EventSelector;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let selectors = match subscription {
            Subscription::All => None,
            Subscription::AnyOf([]) => return Err(SubscriberError::InvalidSubscription),
            Subscription::AnyOf(selectors) => Some(
                selectors
                    .iter()
                    .map(|selector| self.selector(selector))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            Subscription::One(selector) => Some(vec![self.selector(selector)?]),
        };
        let cloud_subscription = match selectors.as_ref() {
            Some(selectors) => Subscription::AnyOf(selectors),
            None => Subscription::All,
        };
        let group = consumer_group;
        let consumer = self
            .subscriber
            .subscribe(group, cloud_subscription)
            .await
            .map_err(|source| match source {
                CloudEventSubscriberError::InvalidSubscription => {
                    SubscriberError::InvalidSubscription
                }
                other => SubscriberError::Subscribe(Box::new(other)),
            })?;
        Ok(EventConsumer::new(
            consumer,
            self.config.type_prefix.clone(),
        ))
    }
}
