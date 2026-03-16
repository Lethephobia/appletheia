use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{SagaDefinition, SagaRunner, SagaWorker, SagaWorkerError};
use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber,
    event::{EventEnvelope, EventSelector},
};

pub struct DefaultSagaWorker<D, S, R> {
    saga_runner: R,
    subscriber: S,
    saga: D,
    stop_requested: AtomicBool,
}

impl<D, S, R> DefaultSagaWorker<D, S, R> {
    pub fn new(saga_runner: R, subscriber: S, saga: D) -> Self {
        Self {
            saga_runner,
            subscriber,
            saga,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, S, R> SagaWorker for DefaultSagaWorker<D, S, R>
where
    D: SagaDefinition,
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: SagaRunner,
{
    type Saga = D;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), SagaWorkerError> {
        let consumer_group = ConsumerGroup::from(D::NAME);

        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, D::SUBSCRIPTION)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            if !D::SUBSCRIPTION.matches(delivery.message()) {
                delivery.ack().await?;
                continue;
            }

            let result = self
                .saga_runner
                .handle_event(&self.saga, delivery.message())
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
