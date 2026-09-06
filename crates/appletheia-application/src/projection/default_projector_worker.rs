use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber,
    event::{EventEnvelope, EventSelector},
};

use super::{Projector, ProjectorRunner, ProjectorSpec, ProjectorWorker, ProjectorWorkerError};

/// Consumes events for projectors passed to `run_forever`.
pub struct DefaultProjectorWorker<S, R> {
    runner: R,
    subscriber: S,
    stop_requested: AtomicBool,
}

impl<S, R> DefaultProjectorWorker<S, R> {
    pub fn new(runner: R, subscriber: S) -> Self {
        Self {
            runner,
            subscriber,
            stop_requested: AtomicBool::new(false),
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
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever<PJ>(&self, projector: &PJ) -> Result<(), ProjectorWorkerError>
    where
        PJ: Projector<Uow = Self::Uow>,
    {
        let descriptor = <PJ::Spec as ProjectorSpec>::DESCRIPTOR;
        let consumer_group = ConsumerGroup::from(descriptor.name);
        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, descriptor.subscription)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            if !descriptor.subscription.matches(delivery.message()) {
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
