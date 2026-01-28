use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{SagaDefinition, SagaRunner, SagaWorker, SagaWorkerError};
use crate::{
    Consumer, ConsumerGroup, Delivery, Topic,
    event::{EventEnvelope, EventSelector},
};

pub struct DefaultSagaWorker<D, T, R> {
    saga_runner: R,
    topic: T,
    saga: D,
    stop_requested: AtomicBool,
}

impl<D, T, R> DefaultSagaWorker<D, T, R> {
    pub fn new(saga_runner: R, topic: T, saga: D) -> Self {
        Self {
            saga_runner,
            topic,
            saga,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, T, R> SagaWorker for DefaultSagaWorker<D, T, R>
where
    D: SagaDefinition,
    T: Topic<EventEnvelope, Selector = EventSelector>,
    T::Consumer: Consumer<EventEnvelope>,
    <T::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
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
            .topic
            .subscribe(&consumer_group, D::EVENTS)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            if !self.saga.matches(delivery.message()) {
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
