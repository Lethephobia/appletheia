use appletheia_application::CloudEvent;
use appletheia_application::messaging::{
    CloudEventPublisher, CloudEventPublisherError, PublishDispatchError, PublishResult,
};
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::Publisher as GooglePublisher;
use google_cloud_pubsub::error::PublishError;

use super::PubsubCloudEventCodec;

#[derive(Clone)]
pub struct PubsubCloudEventPublisher {
    publisher: GooglePublisher,
}

impl PubsubCloudEventPublisher {
    pub fn new(publisher: GooglePublisher) -> Self {
        Self { publisher }
    }

    fn dispatch_error(error: PublishError) -> PublishDispatchError {
        match error {
            PublishError::Rpc(source) => {
                let code = source
                    .status()
                    .map(|status| status.code.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
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
                code: "publish_error".to_string(),
                message: other.to_string(),
            },
        }
    }
}

impl CloudEventPublisher for PubsubCloudEventPublisher {
    async fn publish<'a, I>(
        &self,
        messages: I,
    ) -> Result<Vec<PublishResult>, CloudEventPublisherError>
    where
        I: IntoIterator<Item = &'a CloudEvent>,
    {
        let pubsub_messages = messages
            .into_iter()
            .map(PubsubCloudEventCodec::encode)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| CloudEventPublisherError::Publish(Box::new(error)))?;

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
                Ok(message_id) => {
                    results.push(PublishResult::Success {
                        input_index,
                        transport_message_id: Some(message_id),
                    });
                }
                Err(error) => {
                    let cause = Self::dispatch_error(error);
                    results.push(PublishResult::Failed { input_index, cause });
                }
            }
        }

        Ok(results)
    }
}
