use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{Saga, SagaRunner, SagaSpec, SagaWorker, SagaWorkerError};
use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber,
    event::{EventEnvelope, EventSelector},
};

pub struct DefaultSagaWorker<SG, S, R> {
    saga_runner: R,
    subscriber: S,
    saga: SG,
    stop_requested: AtomicBool,
}

impl<SG, S, R> DefaultSagaWorker<SG, S, R> {
    pub fn new(saga_runner: R, subscriber: S, saga: SG) -> Self {
        Self {
            saga_runner,
            subscriber,
            saga,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<SG, S, R> SagaWorker for DefaultSagaWorker<SG, S, R>
where
    SG: Saga,
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: SagaRunner,
{
    type Saga = SG;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), SagaWorkerError> {
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
