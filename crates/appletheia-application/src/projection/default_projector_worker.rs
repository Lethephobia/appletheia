use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber,
    event::{EventEnvelope, EventSelector},
};

use super::{ProjectorDefinition, ProjectorRunner, ProjectorWorker, ProjectorWorkerError};

pub struct DefaultProjectorWorker<D, S, R> {
    runner: R,
    subscriber: S,
    projector: D,
    stop_requested: AtomicBool,
}

impl<D, S, R> DefaultProjectorWorker<D, S, R> {
    pub fn new(runner: R, subscriber: S, projector: D) -> Self {
        Self {
            runner,
            subscriber,
            projector,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, S, R> ProjectorWorker for DefaultProjectorWorker<D, S, R>
where
    D: ProjectorDefinition,
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: ProjectorRunner<Uow = D::Uow>,
{
    type Projector = D;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), ProjectorWorkerError> {
        let consumer_group = ConsumerGroup::from(D::NAME);
        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, D::SUBSCRIPTION)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            if !D::SUBSCRIPTION.matches(delivery.message()) {
                delivery.ack().await?;
                continue;
            }

            let result = self
                .runner
                .project(&self.projector, delivery.message())
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
