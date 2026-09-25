use super::PubsubMessageCodec;
use super::PubsubTopicName;
use appletheia_application::ConsumerGroup;
use appletheia_application::Subscriber;
use appletheia_application::SubscriberError;
use appletheia_application::messaging::Subscription;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::{Subscriber as GoogleSubscriber, SubscriptionAdmin};
use google_cloud_pubsub::model::Subscription as PubsubSubscription;
use std::sync::Arc;

use super::PubsubSubscriptionPathPrefix;
use super::pubsub_consumer::PubsubConsumer;

pub struct PubsubSubscriber<C>
where
    C: PubsubMessageCodec,
{
    subscriber: GoogleSubscriber,
    subscription_admin: SubscriptionAdmin,
    subscription_path_prefix: PubsubSubscriptionPathPrefix,
    topic_name: PubsubTopicName,
    codec: Arc<C>,
}

impl<C> PubsubSubscriber<C>
where
    C: PubsubMessageCodec,
{
    pub fn new(
        subscriber: GoogleSubscriber,
        subscription_admin: SubscriptionAdmin,
        subscription_path_prefix: PubsubSubscriptionPathPrefix,
        topic_name: PubsubTopicName,
        codec: C,
    ) -> Self {
        Self {
            subscriber,
            subscription_admin,
            subscription_path_prefix,
            topic_name,
            codec: Arc::new(codec),
        }
    }

    fn filter_expression_for_selector(
        &self,
        selector: &C::Selector,
    ) -> Result<String, SubscriberError> {
        self.codec
            .encode_selector(selector)
            .map(|expression| format!("({expression})"))
            .map_err(|source| SubscriberError::Subscribe(Box::new(source)))
    }

    fn filter_expression_for_selectors(
        &self,
        selectors: &[C::Selector],
    ) -> Result<String, SubscriberError> {
        selectors
            .iter()
            .map(|selector| self.filter_expression_for_selector(selector))
            .collect::<Result<Vec<_>, _>>()
            .map(|expressions| expressions.join(" OR "))
    }
}

impl<C> Subscriber<C::Message> for PubsubSubscriber<C>
where
    C: PubsubMessageCodec,
{
    type Consumer = PubsubConsumer<C>;
    type Selector = C::Selector;

    async fn subscribe(
        &self,
        consumer_group: &ConsumerGroup,
        subscription: Subscription<'_, Self::Selector>,
    ) -> Result<Self::Consumer, SubscriberError> {
        let subscription_name = self
            .subscription_path_prefix
            .subscription_name(&self.topic_name, consumer_group);
        let filter = match subscription {
            Subscription::All => String::new(),
            Subscription::AnyOf([]) => {
                return Err(SubscriberError::InvalidSubscription);
            }
            Subscription::AnyOf(selectors) => self.filter_expression_for_selectors(selectors)?,
            Subscription::One(selector) => self.filter_expression_for_selector(selector)?,
        };

        let create_request = PubsubSubscription::new()
            .set_name(&subscription_name)
            .set_topic(self.topic_name.value())
            .set_enable_message_ordering(true)
            .set_filter(filter.clone());

        match self
            .subscription_admin
            .create_subscription()
            .with_request(create_request)
            .send()
            .await
        {
            Ok(_) => {}
            Err(error)
                if matches!(
                    error.status().map(|status| status.code),
                    Some(Code::AlreadyExists)
                ) =>
            {
                let existing = self
                    .subscription_admin
                    .get_subscription()
                    .set_subscription(&subscription_name)
                    .send()
                    .await
                    .map_err(|source| SubscriberError::Subscribe(Box::new(source)))?;
                if existing.topic != self.topic_name.value() || existing.filter != filter {
                    return Err(SubscriberError::SubscriptionConflict);
                }
            }
            Err(error) => {
                return Err(SubscriberError::Subscribe(Box::new(error)));
            }
        }

        let stream = self.subscriber.subscribe(subscription_name).build();
        Ok(PubsubConsumer::new(stream, Arc::clone(&self.codec)))
    }
}
