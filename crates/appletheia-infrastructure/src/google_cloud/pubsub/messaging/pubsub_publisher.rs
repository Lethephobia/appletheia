use super::PubsubMessageCodec;
use appletheia_application::messaging::{
    PublishDispatchError, PublishResult, Publisher, PublisherError,
};
use google_cloud_gax::error::rpc::Code;
use google_cloud_pubsub::client::Publisher as GooglePublisher;
use google_cloud_pubsub::error::PublishError;

#[derive(Clone)]
pub struct PubsubPublisher<C>
where
    C: PubsubMessageCodec,
{
    publisher: GooglePublisher,
    codec: C,
}

impl<C> PubsubPublisher<C>
where
    C: PubsubMessageCodec,
{
    pub fn new(publisher: GooglePublisher, codec: C) -> Self {
        Self { publisher, codec }
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

impl<C> Publisher for PubsubPublisher<C>
where
    C: PubsubMessageCodec,
{
    type Message = C::Message;

    async fn publish<'a, I>(&self, messages: I) -> Result<Vec<PublishResult>, PublisherError>
    where
        I: IntoIterator<Item = &'a C::Message>,
        C::Message: 'a,
    {
        let pubsub_messages = messages
            .into_iter()
            .map(|message| {
                self.codec
                    .encode(message)
                    .map_err(|source| PublisherError::Publish(Box::new(source)))
            })
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
