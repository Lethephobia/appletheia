use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use super::{
    SagaConsumer, SagaDefinition, SagaDelivery, SagaRunner, SagaWorker, SagaWorkerError,
};

pub struct DefaultSagaWorker<D, C, R> {
    saga_runner: R,
    consumer: C,
    saga: D,
    stop_requested: AtomicBool,
}

impl<D, C, R> DefaultSagaWorker<D, C, R> {
    pub fn new(saga_runner: R, consumer: C, saga: D) -> Self {
        Self {
            saga_runner,
            consumer,
            saga,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, C, R> SagaWorker for DefaultSagaWorker<D, C, R>
where
    D: SagaDefinition,
    C: SagaConsumer<Saga = D>,
    R: SagaRunner,
{
    type Uow = R::Uow;
    type Saga = D;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self, uow: &mut Self::Uow) -> Result<(), SagaWorkerError> {
        while !self.is_stop_requested() {
            let mut delivery = self
                .consumer
                .next()
                .await
                .map_err(|source| SagaWorkerError::ConsumerNext(Box::new(source)))?;

            if !self.saga.matches(delivery.event()) {
                delivery
                    .ack()
                    .await
                    .map_err(|source| SagaWorkerError::ConsumerAck(Box::new(source)))?;
                continue;
            }

            let result = self
                .saga_runner
                .handle_event(uow, &self.saga, delivery.event())
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
