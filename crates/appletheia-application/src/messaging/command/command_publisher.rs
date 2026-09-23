use super::CommandPublisherConfig;
use crate::command::CommandEnvelope;
use crate::messaging::PublishableMessage;
use crate::messaging::{CloudEventPublisher, PublishResult, Publisher, PublisherError};

pub struct CommandPublisher<P>
where
    P: CloudEventPublisher,
{
    cloud_event_publisher: P,
    config: CommandPublisherConfig,
}

impl<P> CommandPublisher<P>
where
    P: CloudEventPublisher,
{
    pub fn new(cloud_event_publisher: P, config: CommandPublisherConfig) -> Self {
        Self {
            cloud_event_publisher,
            config,
        }
    }
}

impl<P> Publisher for CommandPublisher<P>
where
    P: CloudEventPublisher,
{
    type Message = CommandEnvelope;

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
        self.cloud_event_publisher
            .publish(&events)
            .await
            .map_err(|source| PublisherError::Publish(Box::new(source)))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::command::{CommandNameOwned, CommandOptions, SerializedCommand};
    use crate::messaging::{CloudEvent, CloudEventPublisherError};
    use crate::request_context::{CausationId, CorrelationId, MessageId};

    use super::*;

    struct RecordingPublisher(Arc<Mutex<Vec<CloudEvent>>>);

    impl CloudEventPublisher for RecordingPublisher {
        async fn publish<'a, I>(
            &self,
            messages: I,
        ) -> Result<Vec<PublishResult>, CloudEventPublisherError>
        where
            I: IntoIterator<Item = &'a CloudEvent>,
        {
            let events = messages.into_iter().cloned().collect::<Vec<_>>();
            let results = events
                .iter()
                .enumerate()
                .map(|(input_index, _)| PublishResult::Success {
                    input_index,
                    transport_message_id: Some(format!("broker-{input_index}")),
                })
                .collect();
            self.0.lock().unwrap().extend(events);
            Ok(results)
        }
    }

    #[tokio::test]
    async fn delegates_borrowed_batch_and_retains_stable_identity_on_republish() {
        let message_id = MessageId::new();
        let first = CommandEnvelope {
            command_name: CommandNameOwned::new("debit".to_owned()).unwrap(),
            command: SerializedCommand::new(serde_json::json!({"amount": 10})).unwrap(),
            message_id,
            correlation_id: CorrelationId::from(message_id.value()),
            causation_id: CausationId::from(message_id),
            options: CommandOptions::default(),
            saga_origin: None,
        };
        let mut second = first.clone();
        second.message_id = MessageId::new();
        let recorded = Arc::new(Mutex::new(Vec::new()));
        let publisher = CommandPublisher::new(
            RecordingPublisher(recorded.clone()),
            CommandPublisherConfig {
                source: "urn:commands".parse().unwrap(),
                type_prefix: Some("example.command".parse().unwrap()),
            },
        );
        let results = publisher.publish([&first, &second]).await.unwrap();
        assert_eq!(
            results,
            vec![
                PublishResult::Success {
                    input_index: 0,
                    transport_message_id: Some("broker-0".to_owned())
                },
                PublishResult::Success {
                    input_index: 1,
                    transport_message_id: Some("broker-1".to_owned())
                },
            ]
        );
        publisher.publish([&first]).await.unwrap();
        let events = recorded.lock().unwrap();
        assert_eq!(events[0].id().as_str(), first.message_id.to_string());
        assert_eq!(events[1].id().as_str(), second.message_id.to_string());
        assert_eq!(events[0], events[2]);
        assert_eq!(events[0].event_type().as_str(), "example.command.debit");
    }
}
