use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{
    SagaConsumer, SagaDefinition, SagaDelivery, SagaRunner, SagaWorker, SagaWorkerError,
};

pub struct DefaultSagaWorker<C, R> {
    saga_runner: R,
    consumer: C,
    stop_requested: AtomicBool,
}

impl<C, R> DefaultSagaWorker<C, R> {
    pub fn new(saga_runner: R, consumer: C) -> Self {
        Self {
            saga_runner,
            consumer,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<C, R> SagaWorker for DefaultSagaWorker<C, R>
where
    C: SagaConsumer,
    R: SagaRunner,
{
    type Uow = R::Uow;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever<D: SagaDefinition>(
        &mut self,
        uow: &mut Self::Uow,
        saga: &D,
    ) -> Result<(), SagaWorkerError> {
        while !self.is_stop_requested() {
            let mut delivery = self
                .consumer
                .next()
                .await
                .map_err(|source| SagaWorkerError::ConsumerNext(Box::new(source)))?;

            if !saga.matches(delivery.event()) {
                delivery
                    .ack()
                    .await
                    .map_err(|source| SagaWorkerError::ConsumerAck(Box::new(source)))?;
                continue;
            }

            let result = self
                .saga_runner
                .handle_event(uow, saga, delivery.event())
                .await;

            match result {
                Ok(_) => delivery
                    .ack()
                    .await
                    .map_err(|source| SagaWorkerError::ConsumerAck(Box::new(source)))?,
                Err(error) => {
                    delivery
                        .nack()
                        .await
                        .map_err(|source| SagaWorkerError::ConsumerNack(Box::new(source)))?;
                    return Err(error.into());
                }
            }
        }

        Ok(())
    }
}
