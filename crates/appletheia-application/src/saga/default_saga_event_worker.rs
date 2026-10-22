use tokio_util::sync::CancellationToken;

use super::{Saga, SagaEventWorker, SagaEventWorkerError, SagaRunner};
use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber, Subscription,
    event::{EventEnvelope, EventSelector},
};

/// Consumes events for sagas passed to `run_forever`.
pub struct DefaultSagaEventWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: SagaRunner,
{
    saga_runner: R,
    subscriber: S,
    stop_token: CancellationToken,
}

impl<S, R> DefaultSagaEventWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
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

impl<S, R> SagaEventWorker for DefaultSagaEventWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
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
    ) -> Result<(), SagaEventWorkerError<SG::HandlerError>> {
        let definition = saga.definition()?;
        let consumer_group = ConsumerGroup::new(format!("saga_event_{}", definition.name()))?;
        let selectors = definition.event_selectors();

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
                .handle_event(&definition, delivery.message())
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
