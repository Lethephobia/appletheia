use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{SagaDefinition, SagaRunner, SagaWorker, SagaWorkerError};
use crate::{
    Consumer, ConsumerFactory, Delivery,
    event::{EventEnvelope, EventSelector},
};

pub struct DefaultSagaWorker<D, B, R> {
    saga_runner: R,
    consumer_factory: B,
    saga: D,
    stop_requested: AtomicBool,
}

impl<D, B, R> DefaultSagaWorker<D, B, R> {
    pub fn new(saga_runner: R, consumer_factory: B, saga: D) -> Self {
        Self {
            saga_runner,
            consumer_factory,
            saga,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, B, R> SagaWorker for DefaultSagaWorker<D, B, R>
where
    D: SagaDefinition,
    B: ConsumerFactory<EventEnvelope, Selector = EventSelector>,
    B::Consumer: Consumer<EventEnvelope>,
    <B::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
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
        let mut consumer = self
            .consumer_factory
            .subscribe(D::NAME.value(), D::EVENTS)
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
