use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{Saga, SagaEventWorker, SagaEventWorkerError, SagaRunner, SagaSpec};
use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber,
    event::{EventEnvelope, EventSelector},
};

/// Consumes events for saga definitions passed to `run_forever`.
pub struct DefaultSagaEventWorker<S, R> {
    saga_runner: R,
    subscriber: S,
    stop_requested: AtomicBool,
}

impl<S, R> DefaultSagaEventWorker<S, R> {
    pub fn new(saga_runner: R, subscriber: S) -> Self {
        Self {
            saga_runner,
            subscriber,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<S, R> SagaEventWorker for DefaultSagaEventWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: SagaRunner,
{
    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever<SG: Saga>(&self, saga: &SG) -> Result<(), SagaEventWorkerError> {
        let descriptor = <SG::Spec as SagaSpec>::DESCRIPTOR;
        let consumer_group = ConsumerGroup::from(descriptor.name);

        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, descriptor.subscription)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            if !descriptor.subscription.matches(delivery.message()) {
                delivery.ack().await?;
                continue;
            }

            let result = self
                .saga_runner
                .handle_event(saga, delivery.message())
                .await;

            match result {
                Ok(_) => delivery.ack().await?,
                Err(error) => {
                    delivery.nack().await?;
                    return Err(error.into());
                }
            }
        }

        Ok(())
    }
}
