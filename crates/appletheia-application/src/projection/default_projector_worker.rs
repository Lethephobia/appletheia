use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::{
    Consumer, ConsumerGroup, Delivery, Topic,
    event::{EventEnvelope, EventSelector},
};

use super::{ProjectorDefinition, ProjectorRunner, ProjectorWorker, ProjectorWorkerError};

pub struct DefaultProjectorWorker<D, T, R> {
    runner: R,
    topic: T,
    projector: D,
    stop_requested: AtomicBool,
}

impl<D, T, R> DefaultProjectorWorker<D, T, R> {
    pub fn new(runner: R, topic: T, projector: D) -> Self {
        Self {
            runner,
            topic,
            projector,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<D, T, R> ProjectorWorker for DefaultProjectorWorker<D, T, R>
where
    D: ProjectorDefinition,
    T: Topic<EventEnvelope, Selector = EventSelector>,
    T::Consumer: Consumer<EventEnvelope>,
    <T::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
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
            .topic
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
