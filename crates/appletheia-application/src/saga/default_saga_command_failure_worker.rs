use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::command::CommandFailureEnvelope;
use crate::messaging::Subscription;
use crate::{Consumer, ConsumerGroup, Delivery, Subscriber};

use super::{
    Saga, SagaCommandFailureWorker, SagaCommandFailureWorkerError, SagaName, SagaRunner, SagaSpec,
};

/// Consumes terminal command failures for one saga definition.
pub struct DefaultSagaCommandFailureWorker<SG, S, R> {
    saga_runner: R,
    subscriber: S,
    saga: SG,
    stop_requested: AtomicBool,
}

impl<SG, S, R> DefaultSagaCommandFailureWorker<SG, S, R> {
    pub fn new(saga_runner: R, subscriber: S, saga: SG) -> Self {
        Self {
            saga_runner,
            subscriber,
            saga,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<SG, S, R> SagaCommandFailureWorker for DefaultSagaCommandFailureWorker<SG, S, R>
where
    SG: Saga,
    S: Subscriber<CommandFailureEnvelope, Selector = SagaName>,
    S::Consumer: Consumer<CommandFailureEnvelope>,
    <S::Consumer as Consumer<CommandFailureEnvelope>>::Delivery: Delivery<CommandFailureEnvelope>,
    R: SagaRunner,
{
    type Saga = SG;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), SagaCommandFailureWorkerError> {
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
                .handle_command_failure(&self.saga, delivery.message())
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
