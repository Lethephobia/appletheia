use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::command::CommandFailureEnvelope;
use crate::messaging::Subscription;
use crate::{Consumer, ConsumerGroup, Delivery, Subscriber};

use super::{
    Saga, SagaCommandFailureWorker, SagaCommandFailureWorkerError, SagaName, SagaRunner, SagaSpec,
};

/// Consumes terminal command failures for saga definitions passed to `run_forever`.
pub struct DefaultSagaCommandFailureWorker<S, R> {
    saga_runner: R,
    subscriber: S,
    stop_requested: AtomicBool,
}

impl<S, R> DefaultSagaCommandFailureWorker<S, R> {
    pub fn new(saga_runner: R, subscriber: S) -> Self {
        Self {
            saga_runner,
            subscriber,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<S, R> SagaCommandFailureWorker for DefaultSagaCommandFailureWorker<S, R>
where
    S: Subscriber<CommandFailureEnvelope, Selector = SagaName>,
    S::Consumer: Consumer<CommandFailureEnvelope>,
    <S::Consumer as Consumer<CommandFailureEnvelope>>::Delivery: Delivery<CommandFailureEnvelope>,
    R: SagaRunner,
{
    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever<SG: Saga>(&self, saga: &SG) -> Result<(), SagaCommandFailureWorkerError> {
        let descriptor = <SG::Spec as SagaSpec>::DESCRIPTOR;
        let consumer_group = ConsumerGroup::from(descriptor.name);
        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, Subscription::One(&descriptor.name))
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;
            let result = self
                .saga_runner
                .handle_command_failure(saga, delivery.message())
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
