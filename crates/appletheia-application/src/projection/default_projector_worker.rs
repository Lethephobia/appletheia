use tokio_util::sync::CancellationToken;

use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber, Subscription,
    event::{EventEnvelope, EventSelector},
};

use super::{Projector, ProjectorRunner, ProjectorWorker, ProjectorWorkerError};

/// Consumes events for projectors passed to `run_forever`.
pub struct DefaultProjectorWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: ProjectorRunner,
{
    runner: R,
    subscriber: S,
    stop_token: CancellationToken,
}

impl<S, R> DefaultProjectorWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: ProjectorRunner,
{
    pub fn new(runner: R, subscriber: S) -> Self {
        Self {
            runner,
            subscriber,
            stop_token: CancellationToken::new(),
        }
    }
}

impl<S, R> ProjectorWorker for DefaultProjectorWorker<S, R>
where
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: ProjectorRunner,
{
    type Uow = R::Uow;

    fn is_stop_requested(&self) -> bool {
        self.stop_token.is_cancelled()
    }

    fn request_graceful_stop(&self) {
        self.stop_token.cancel();
    }

    async fn run_forever<PJ>(&self, projector: &PJ) -> Result<(), ProjectorWorkerError>
    where
        PJ: Projector<Uow = Self::Uow>,
    {
        let selectors = PJ::EVENT_SELECTORS;
        if selectors.is_empty() {
            return Ok(());
        }
        let subscription = Subscription::AnyOf(selectors);
        let consumer_group = ConsumerGroup::new(format!("projector_{}", PJ::NAME))?;
        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, subscription)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = tokio::select! {
                biased;
                _ = self.stop_token.cancelled() => break,
                delivery = consumer.next() => delivery?,
            };

            if !subscription.matches(delivery.message()) {
                delivery.ack().await?;
            } else {
                let result = self.runner.project(projector, delivery.message()).await;

                match result {
                    Ok(_) => delivery.ack().await?,
                    Err(error) => {
                        delivery.nack().await?;
                        return Err(error.into());
                    }
                }
            }
        }

        Ok(())
    }
}
