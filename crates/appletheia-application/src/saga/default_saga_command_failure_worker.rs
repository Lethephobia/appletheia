use tokio_util::sync::CancellationToken;

use crate::command::{CommandFailureEnvelope, CommandFailureSelector};
use crate::messaging::Subscription;
use crate::{Consumer, ConsumerGroup, Delivery, Subscriber};

use super::{Saga, SagaCommandFailureWorker, SagaCommandFailureWorkerError, SagaRunner};

/// Consumes terminal command failures for sagas passed to `run_forever`.
pub struct DefaultSagaCommandFailureWorker<S, R>
where
    S: Subscriber<CommandFailureEnvelope, Selector = CommandFailureSelector>,
    S::Consumer: Consumer<CommandFailureEnvelope>,
    <S::Consumer as Consumer<CommandFailureEnvelope>>::Delivery: Delivery<CommandFailureEnvelope>,
    R: SagaRunner,
{
    saga_runner: R,
    subscriber: S,
    stop_token: CancellationToken,
}

impl<S, R> DefaultSagaCommandFailureWorker<S, R>
where
    S: Subscriber<CommandFailureEnvelope, Selector = CommandFailureSelector>,
    S::Consumer: Consumer<CommandFailureEnvelope>,
    <S::Consumer as Consumer<CommandFailureEnvelope>>::Delivery: Delivery<CommandFailureEnvelope>,
    R: SagaRunner,
{
    pub fn new(saga_runner: R, subscriber: S) -> Self {
        Self {
            saga_runner,
            subscriber,
            stop_token: CancellationToken::new(),
        }
    }
}

impl<S, R> SagaCommandFailureWorker for DefaultSagaCommandFailureWorker<S, R>
where
    S: Subscriber<CommandFailureEnvelope, Selector = CommandFailureSelector>,
    S::Consumer: Consumer<CommandFailureEnvelope>,
    <S::Consumer as Consumer<CommandFailureEnvelope>>::Delivery: Delivery<CommandFailureEnvelope>,
    R: SagaRunner,
{
    fn is_stop_requested(&self) -> bool {
        self.stop_token.is_cancelled()
    }

    fn request_graceful_stop(&self) {
        self.stop_token.cancel();
    }

    async fn run_forever<SG: Saga>(
        &self,
        saga: &SG,
    ) -> Result<(), SagaCommandFailureWorkerError<SG::HandlerError>> {
        let definition = saga.definition()?;
        let consumer_group =
            ConsumerGroup::new(format!("saga_command_failure_{}", definition.name()))?;
        let selectors = definition.command_failure_selectors();
        if selectors.is_empty() {
            return Ok(());
        }
        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, Subscription::AnyOf(&selectors))
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = tokio::select! {
                biased;
                _ = self.stop_token.cancelled() => break,
                delivery = consumer.next() => delivery?,
            };
            if !Subscription::AnyOf(&selectors).matches(delivery.message()) {
                delivery.ack().await?;
                continue;
            }
            let result = self
                .saga_runner
                .handle_command_failure(&definition, delivery.message())
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
