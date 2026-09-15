use std::collections::HashMap;

use appletheia_application::messaging::{
    PublishDispatchError, PublishResult, PublishableMessage, Publisher, PublisherError,
};
use appletheia_application::read_model::ReadModelInvalidationEnvelope;
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::Publisher as GooglePublisher;
use google_cloud_pubsub::error::PublishError;
use google_cloud_pubsub::model::Message;

/// Publishes read-model invalidations to Google Cloud Pub/Sub.
#[derive(Clone)]
pub struct PubsubReadModelInvalidationPublisher {
    publisher: GooglePublisher,
}

impl PubsubReadModelInvalidationPublisher {
    pub fn new(publisher: GooglePublisher) -> Self {
        Self { publisher }
    }

    fn build_message(
        &self,
        invalidation: &ReadModelInvalidationEnvelope,
    ) -> Result<Message, PublisherError> {
        let mut attributes = HashMap::new();
        attributes.insert(
            "source_projector_name".to_owned(),
            invalidation.source_projector_name.to_string(),
        );
        attributes.insert(
            "source_event_sequence".to_owned(),
            invalidation.source_event_sequence.to_string(),
        );
        attributes.insert(
            "source_event_id".to_owned(),
            invalidation.source_event_id.to_string(),
        );

        let data = serde_json::to_vec(invalidation)
            .map_err(|source| PublisherError::Publish(Box::new(source)))?;

        Ok(Message::new()
            .set_data(data)
            .set_attributes(attributes)
            .set_ordering_key(invalidation.ordering_key().to_string()))
    }

    fn dispatch_error(error: PublishError) -> PublishDispatchError {
        match error {
            PublishError::Rpc(source) => {
                let code = source
                    .status()
                    .map(|status| status.code.to_string())
                    .unwrap_or_else(|| "unknown".to_owned());
                let message = source.to_string();

                match source.status().map(|status| status.code) {
                    Some(
                        Code::Unavailable
                        | Code::DeadlineExceeded
                        | Code::ResourceExhausted
                        | Code::Aborted,
                    ) => PublishDispatchError::Transient { code, message },
                    _ => PublishDispatchError::Permanent { code, message },
                }
            }
            other => PublishDispatchError::Permanent {
                code: "publish_error".to_owned(),
                message: other.to_string(),
            },
        }
    }
}

impl Publisher<ReadModelInvalidationEnvelope> for PubsubReadModelInvalidationPublisher {
    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a ReadModelInvalidationEnvelope>,
        ReadModelInvalidationEnvelope: 'a,
    {
        let pubsub_messages = messages
            .into_iter()
            .map(|change| self.build_message(change))
            .collect::<Result<Vec<_>, _>>()?;

        if pubsub_messages.is_empty() {
            return Ok(Vec::new());
        }

        let publish_futures = pubsub_messages
            .into_iter()
            .map(|message| self.publisher.publish(message))
            .collect::<Vec<_>>();
        let mut results = Vec::with_capacity(publish_futures.len());

        for (input_index, publish_future) in publish_futures.into_iter().enumerate() {
            match publish_future.await {
                Ok(message_id) => results.push(PublishResult::Success {
                    input_index,
                    transport_message_id: Some(message_id),
                }),
                Err(error) => results.push(PublishResult::Failed {
                    input_index,
                    cause: Self::dispatch_error(error),
                }),
            }
        }

        Ok(results)
    }
}
