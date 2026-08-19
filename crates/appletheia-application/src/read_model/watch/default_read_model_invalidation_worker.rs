use std::sync::atomic::{AtomicBool, Ordering};

use crate::messaging::{Consumer, ConsumerGroup, Delivery, Selector, Subscriber, Subscription};
use crate::read_model::ReadModelInvalidationEnvelope;

use super::{
    ReadModelInvalidationDispatcher, ReadModelInvalidationShard, ReadModelInvalidationWorker,
    ReadModelInvalidationWorkerError,
};

/// Consumes one fixed invalidation shard and acknowledges successful refresh fanout.
pub struct DefaultReadModelInvalidationWorker<S, D> {
    subscriber: S,
    dispatcher: D,
    consumer_group: ConsumerGroup,
    shard: ReadModelInvalidationShard,
    stop_requested: AtomicBool,
}

impl<S, D> DefaultReadModelInvalidationWorker<S, D> {
    pub fn new(
        subscriber: S,
        dispatcher: D,
        consumer_group: ConsumerGroup,
        shard: ReadModelInvalidationShard,
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

impl<S, D> DefaultReadModelInvalidationWorker<S, D>
where
    D: ReadModelInvalidationDispatcher,
{
    async fn process_delivery<DL>(
        &self,
        delivery: &mut DL,
    ) -> Result<(), ReadModelInvalidationWorkerError>
    where
        DL: Delivery<ReadModelInvalidationEnvelope>,
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

impl<S, D> ReadModelInvalidationWorker for DefaultReadModelInvalidationWorker<S, D>
where
    S: Subscriber<ReadModelInvalidationEnvelope, Selector = ReadModelInvalidationShard>,
    S::Consumer: Consumer<ReadModelInvalidationEnvelope>,
    <S::Consumer as Consumer<ReadModelInvalidationEnvelope>>::Delivery:
        Delivery<ReadModelInvalidationEnvelope>,
    D: ReadModelInvalidationDispatcher,
{
    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), ReadModelInvalidationWorkerError> {
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
    use std::num::NonZeroU32;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use appletheia_domain::{AggregateVersion, EventId, EventOccurredAt};
    use serde_json::json;
    use uuid::Uuid;

    use crate::event::{
        AggregateIdValue, AggregateTypeOwned, EventEnvelope, EventNameOwned, EventSequence,
        SerializedEventPayload,
    };
    use crate::messaging::{ConsumerError, Delivery};
    use crate::projection::ProjectorName;
    use crate::read_model::{ReadModelDependency, SerializedPartition};
    use crate::request_context::{
        CausationId, CorrelationId, MessageId, Principal, RequestContext,
    };

    use super::*;
    use crate::read_model::watch::ReadModelWatchRegistryError;

    struct TestDispatcher {
        fail: bool,
    }

    impl ReadModelInvalidationDispatcher for TestDispatcher {
        async fn dispatch(
            &self,
            _envelope: &ReadModelInvalidationEnvelope,
        ) -> Result<(), ReadModelWatchRegistryError> {
            if self.fail {
                Err(ReadModelWatchRegistryError::DeliveryBackpressureExceeded)
            } else {
                Ok(())
            }
        }
    }

    struct TestDelivery {
        message: ReadModelInvalidationEnvelope,
        acked: Arc<AtomicBool>,
        nacked: Arc<AtomicBool>,
    }

    impl Delivery<ReadModelInvalidationEnvelope> for TestDelivery {
        fn message(&self) -> &ReadModelInvalidationEnvelope {
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

    fn envelope() -> ReadModelInvalidationEnvelope {
        let correlation_id = CorrelationId::from(Uuid::now_v7());
        let message_id = MessageId::new();
        let event = EventEnvelope {
            event_sequence: EventSequence::try_from(1).expect("sequence should be valid"),
            event_id: EventId::new(),
            aggregate_type: AggregateTypeOwned::try_from("test")
                .expect("aggregate type should be valid"),
            aggregate_id: AggregateIdValue::from(Uuid::now_v7()),
            aggregate_version: AggregateVersion::try_from(1).expect("version should be valid"),
            event_name: EventNameOwned::try_from("changed").expect("event name should be valid"),
            payload: SerializedEventPayload::try_from(json!({})).expect("payload should be valid"),
            occurred_at: EventOccurredAt::now(),
            correlation_id,
            causation_id: CausationId::from(message_id),
            context: RequestContext::new(correlation_id, message_id, Principal::System)
                .expect("request context should be valid"),
        };
        let dependency = ReadModelDependency::Partition(
            SerializedPartition::try_from(json!({ "fragment": "test", "key": 1 }))
                .expect("partition should be valid"),
        );
        ReadModelInvalidationEnvelope::try_new(
            &event,
            ProjectorName::new("test_projector"),
            [dependency],
        )
        .expect("invalidation should be valid")
    }

    fn delivery(
        message: ReadModelInvalidationEnvelope,
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
    async fn acknowledges_a_successful_invalidation_dispatch() {
        let message = envelope();
        let shard = ReadModelInvalidationShard::for_envelope(
            &message,
            NonZeroU32::new(64).expect("shard count should be nonzero"),
        );
        let worker = DefaultReadModelInvalidationWorker::new(
            (),
            TestDispatcher { fail: false },
            crate::messaging::ConsumerGroup::new("read_model_watch".to_owned())
                .expect("consumer group should be valid"),
            shard,
        );
        let (mut delivery, acked, nacked) = delivery(message);

        worker
            .process_delivery(&mut delivery)
            .await
            .expect("dispatch should succeed");

        assert!(acked.load(Ordering::SeqCst));
        assert!(!nacked.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn negatively_acknowledges_a_failed_invalidation_dispatch() {
        let message = envelope();
        let shard = ReadModelInvalidationShard::for_envelope(
            &message,
            NonZeroU32::new(64).expect("shard count should be nonzero"),
        );
        let worker = DefaultReadModelInvalidationWorker::new(
            (),
            TestDispatcher { fail: true },
            crate::messaging::ConsumerGroup::new("read_model_watch".to_owned())
                .expect("consumer group should be valid"),
            shard,
        );
        let (mut delivery, acked, nacked) = delivery(message);

        let error = worker
            .process_delivery(&mut delivery)
            .await
            .expect_err("dispatch should fail");

        assert!(matches!(
            error,
            ReadModelInvalidationWorkerError::Dispatch(_)
        ));
        assert!(!acked.load(Ordering::SeqCst));
        assert!(nacked.load(Ordering::SeqCst));
    }
}
