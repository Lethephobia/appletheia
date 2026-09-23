use super::CommandDelivery;
use crate::command::CommandEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{
    CloudEventConsumer, CloudEventDelivery, CloudEventTypePrefix, Consumer, ConsumerError,
};

pub struct CommandConsumer<C>
where
    C: CloudEventConsumer,
{
    consumer: C,
    type_prefix: Option<CloudEventTypePrefix>,
}

impl<C> CommandConsumer<C>
where
    C: CloudEventConsumer,
{
    pub fn new(consumer: C, type_prefix: Option<CloudEventTypePrefix>) -> Self {
        Self {
            consumer,
            type_prefix,
        }
    }
}

impl<C> Consumer<CommandEnvelope> for CommandConsumer<C>
where
    C: CloudEventConsumer,
{
    type Delivery = CommandDelivery<C::Delivery>;

    async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
        let mut delivery = self
            .consumer
            .next()
            .await
            .map_err(|source| ConsumerError::Next(Box::new(source)))?;
        match CommandEnvelope::try_from_cloud_event(delivery.message(), self.type_prefix.as_ref()) {
            Ok(message) => Ok(CommandDelivery::new(delivery, message)),
            Err(source) => {
                delivery
                    .nack()
                    .await
                    .map_err(|nack_error| ConsumerError::Next(Box::new(nack_error)))?;
                Err(ConsumerError::Next(Box::new(source)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    use crate::command::{CommandNameOwned, CommandOptions, SerializedCommand};
    use crate::messaging::{
        CloudEvent, CloudEventConsumerError, CloudEventDeliveryError, Delivery,
    };
    use crate::request_context::{CausationId, CorrelationId, MessageId};

    use super::*;

    struct TestDelivery {
        event: CloudEvent,
        acknowledgements: Arc<Mutex<Vec<&'static str>>>,
    }

    impl CloudEventDelivery for TestDelivery {
        fn message(&self) -> &CloudEvent {
            &self.event
        }

        async fn ack(&mut self) -> Result<(), CloudEventDeliveryError> {
            self.acknowledgements.lock().unwrap().push("ack");
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), CloudEventDeliveryError> {
            self.acknowledgements.lock().unwrap().push("nack");
            Ok(())
        }
    }

    struct TestConsumer(VecDeque<TestDelivery>);

    impl CloudEventConsumer for TestConsumer {
        type Delivery = TestDelivery;

        async fn next(&mut self) -> Result<Self::Delivery, CloudEventConsumerError> {
            Ok(self.0.pop_front().unwrap())
        }
    }

    #[tokio::test]
    async fn conversion_failures_nack_and_successful_deliveries_delegate_acknowledgement() {
        let message_id = MessageId::new();
        let envelope = CommandEnvelope {
            command_name: CommandNameOwned::new("transfer".to_owned()).unwrap(),
            command: SerializedCommand::new(serde_json::json!({"amount": 10})).unwrap(),
            message_id,
            correlation_id: CorrelationId::from(message_id.value()),
            causation_id: CausationId::from(message_id),
            options: CommandOptions::default(),
            saga_origin: None,
        };
        let event = envelope
            .try_to_cloud_event(&"urn:commands".parse().unwrap(), None)
            .unwrap();
        let mut invalid = event.clone();
        invalid.remove_extension(&"correlationid".parse().unwrap());
        let acknowledgements = Arc::new(Mutex::new(Vec::new()));
        let deliveries = [invalid, event.clone(), event]
            .into_iter()
            .map(|event| TestDelivery {
                event,
                acknowledgements: acknowledgements.clone(),
            })
            .collect();
        let mut consumer = CommandConsumer::new(TestConsumer(deliveries), None);
        assert!(consumer.next().await.is_err());
        assert_eq!(*acknowledgements.lock().unwrap(), vec!["nack"]);
        let mut delivered = consumer.next().await.unwrap();
        assert_eq!(delivered.message(), &envelope);
        assert_eq!(*acknowledgements.lock().unwrap(), vec!["nack"]);
        delivered.ack().await.unwrap();
        let mut retry = consumer.next().await.unwrap();
        retry.nack().await.unwrap();
        assert_eq!(
            *acknowledgements.lock().unwrap(),
            vec!["nack", "ack", "nack"]
        );
    }
}
