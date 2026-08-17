use std::sync::atomic::{AtomicBool, Ordering};

use crate::messaging::{Consumer, ConsumerGroup, Delivery, Selector, Subscriber, Subscription};
use crate::read_model::ReadModelFragmentChangeEnvelope;

use super::{
    ReadModelFragmentChangeDispatcher, ReadModelFragmentChangeShard, ReadModelFragmentChangeWorker,
    ReadModelFragmentChangeWorkerError,
};

/// Consumes one fixed source-partition shard and acknowledges successful fanout.
pub struct DefaultReadModelFragmentChangeWorker<S, D> {
    subscriber: S,
    dispatcher: D,
    consumer_group: ConsumerGroup,
    shard: ReadModelFragmentChangeShard,
    stop_requested: AtomicBool,
}

impl<S, D> DefaultReadModelFragmentChangeWorker<S, D> {
    pub fn new(
        subscriber: S,
        dispatcher: D,
        consumer_group: ConsumerGroup,
        shard: ReadModelFragmentChangeShard,
    ) -> Self {
        Self {
            subscriber,
            dispatcher,
            consumer_group,
            shard,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<S, D> DefaultReadModelFragmentChangeWorker<S, D>
where
    D: ReadModelFragmentChangeDispatcher,
{
    async fn process_delivery<DL>(
        &self,
        delivery: &mut DL,
    ) -> Result<(), ReadModelFragmentChangeWorkerError>
    where
        DL: Delivery<ReadModelFragmentChangeEnvelope>,
    {
        if !self.shard.matches(delivery.message()) {
            delivery.ack().await?;
            return Ok(());
        }
        match self.dispatcher.dispatch(delivery.message()).await {
            Ok(()) => delivery.ack().await?,
            Err(error) => {
                delivery.nack().await?;
                return Err(error.into());
            }
        }
        Ok(())
    }
}

impl<S, D> ReadModelFragmentChangeWorker for DefaultReadModelFragmentChangeWorker<S, D>
where
    S: Subscriber<ReadModelFragmentChangeEnvelope, Selector = ReadModelFragmentChangeShard>,
    S::Consumer: Consumer<ReadModelFragmentChangeEnvelope>,
    <S::Consumer as Consumer<ReadModelFragmentChangeEnvelope>>::Delivery:
        Delivery<ReadModelFragmentChangeEnvelope>,
    D: ReadModelFragmentChangeDispatcher,
{
    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), ReadModelFragmentChangeWorkerError> {
        let mut consumer = self
            .subscriber
            .subscribe(&self.consumer_group, Subscription::One(&self.shard))
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;
            self.process_delivery(&mut delivery).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::num::NonZeroU32;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::DefaultReadModelFragmentChangeWorker;
    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use crate::messaging::{ConsumerError, ConsumerGroup, Delivery};
    use crate::projection::ProjectorName;
    use crate::read_model::{
        ReadModelFragment, ReadModelFragmentChange, ReadModelFragmentChangeEnvelope,
        ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
    };
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };

    use crate::read_model::watch::{
        ReadModelFragmentChangeDispatcher, ReadModelFragmentChangeShard,
        ReadModelWatchDeliveryError, ReadModelWatchDispatchError,
        ReadModelWatchFragmentDispatcherError, ReadModelWatchSessionId,
    };

    #[derive(Clone, Deserialize, Serialize)]
    struct TestFragment {
        id: Uuid,
    }

    impl ReadModelObservationSource for TestFragment {
        fn observations(&self) -> Vec<ReadModelObservation> {
            Vec::new()
        }
    }

    impl ReadModelFragment for TestFragment {
        const NAME: ReadModelFragmentName = ReadModelFragmentName::new("test_fragment");
        type Key = Uuid;

        fn key(&self) -> Self::Key {
            self.id
        }
    }

    struct TestDispatcher {
        fail: bool,
    }

    impl ReadModelFragmentChangeDispatcher for TestDispatcher {
        async fn dispatch(
            &self,
            _envelope: &ReadModelFragmentChangeEnvelope,
        ) -> Result<(), ReadModelWatchFragmentDispatcherError> {
            if self.fail {
                return Err(ReadModelWatchFragmentDispatcherError {
                    session_id: ReadModelWatchSessionId::new(),
                    source: ReadModelWatchDispatchError::Delivery(
                        ReadModelWatchDeliveryError::new(io::Error::other("delivery failed")),
                    ),
                });
            }
            Ok(())
        }
    }

    struct TestDelivery {
        message: ReadModelFragmentChangeEnvelope,
        acked: Arc<AtomicBool>,
        nacked: Arc<AtomicBool>,
    }

    impl Delivery<ReadModelFragmentChangeEnvelope> for TestDelivery {
        fn message(&self) -> &ReadModelFragmentChangeEnvelope {
            &self.message
        }

        async fn ack(&mut self) -> Result<(), ConsumerError> {
            self.acked.store(true, Ordering::SeqCst);
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), ConsumerError> {
            self.nacked.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    fn envelope() -> ReadModelFragmentChangeEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::new();
        let event = EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("test_item")
                .expect("aggregate type should be valid"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
            event_name: EventNameOwned::try_from("changed").expect("event name should be valid"),
            payload: SerializedEventPayload::try_from(serde_json::json!({}))
                .expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System)
                .expect("request context should be valid"),
        };
        let change =
            ReadModelFragmentChange::try_from_fragment(&TestFragment { id: Uuid::now_v7() })
                .expect("fragment should serialize");
        ReadModelFragmentChangeEnvelope::from_changes(
            vec![change],
            &event,
            ProjectorName::new("test_projector"),
        )
        .expect("fragment change should finalize")
    }

    fn delivery(
        message: ReadModelFragmentChangeEnvelope,
    ) -> (TestDelivery, Arc<AtomicBool>, Arc<AtomicBool>) {
        let acked = Arc::new(AtomicBool::new(false));
        let nacked = Arc::new(AtomicBool::new(false));
        (
            TestDelivery {
                message,
                acked: Arc::clone(&acked),
                nacked: Arc::clone(&nacked),
            },
            acked,
            nacked,
        )
    }

    #[tokio::test]
    async fn acknowledges_successful_dispatch_including_no_active_sessions() {
        let message = envelope();
        let shard = ReadModelFragmentChangeShard::for_envelope(
            &message,
            NonZeroU32::new(64).expect("shard count should be nonzero"),
        );
        let worker = DefaultReadModelFragmentChangeWorker::new(
            (),
            TestDispatcher { fail: false },
            ConsumerGroup::new("read_model_watch".to_owned())
                .expect("consumer group should be valid"),
            shard,
        );
        let (mut delivery, acked, nacked) = delivery(message);

        worker
            .process_delivery(&mut delivery)
            .await
            .expect("successful dispatch should be acknowledged");

        assert!(acked.load(Ordering::SeqCst));
        assert!(!nacked.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn negatively_acknowledges_delivery_failure_for_retry() {
        let message = envelope();
        let shard = ReadModelFragmentChangeShard::for_envelope(
            &message,
            NonZeroU32::new(64).expect("shard count should be nonzero"),
        );
        let worker = DefaultReadModelFragmentChangeWorker::new(
            (),
            TestDispatcher { fail: true },
            ConsumerGroup::new("read_model_watch".to_owned())
                .expect("consumer group should be valid"),
            shard,
        );
        let (mut delivery, acked, nacked) = delivery(message);

        let error = worker
            .process_delivery(&mut delivery)
            .await
            .expect_err("delivery failure should be retried");

        assert!(matches!(
            error,
            crate::read_model::watch::ReadModelFragmentChangeWorkerError::Dispatch(_)
        ));
        assert!(!acked.load(Ordering::SeqCst));
        assert!(nacked.load(Ordering::SeqCst));
    }
}
