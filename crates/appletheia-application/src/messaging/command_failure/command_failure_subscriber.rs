use super::{CommandFailureConsumer, CommandFailureSubscriberConfig};
use crate::command::CommandFailureEnvelope;
use crate::messaging::{
    CloudEventSelector, CloudEventSubscriber, CloudEventSubscriberError, ConsumerGroup, Subscriber,
    SubscriberError, Subscription,
};
use crate::saga::SagaName;

pub struct CommandFailureSubscriber<S> {
    subscriber: S,
    config: CommandFailureSubscriberConfig,
}

impl<S> CommandFailureSubscriber<S> {
    pub fn new(subscriber: S, config: CommandFailureSubscriberConfig) -> Self {
        Self { subscriber, config }
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

impl<S: CloudEventSubscriber> Subscriber<CommandFailureEnvelope> for CommandFailureSubscriber<S> {
    type Consumer = CommandFailureConsumer<S::Consumer>;
    type Selector = SagaName;

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
        let failure_group =
            ConsumerGroup::new(format!("{}_command_failures", consumer_group.value()))
                .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?;
        let group = &failure_group;
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
        Ok(CommandFailureConsumer::new(
            consumer,
            self.config.type_prefix.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saga_subscription_uses_extension_without_subject_or_type() {
        let subscriber = CommandFailureSubscriber::new(
            (),
            CommandFailureSubscriberConfig {
                type_prefix: Some("example.command".parse().unwrap()),
            },
        );
        let selector = subscriber.selector(&SagaName::new("transfer")).unwrap();
        assert!(selector.event_type.is_none());
        assert!(selector.subject.is_none());
        assert_eq!(
            selector
                .extensions
                .get(&"saganame".parse().unwrap())
                .unwrap()
                .as_str(),
            "transfer"
        );
    }
}
