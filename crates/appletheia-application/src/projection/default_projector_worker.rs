use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::{
    Consumer, ConsumerGroup, Delivery, Subscriber,
    event::{EventEnvelope, EventSelector},
};

use super::{Projector, ProjectorRunner, ProjectorSpec, ProjectorWorker, ProjectorWorkerError};

pub struct DefaultProjectorWorker<PJ, S, R> {
    runner: R,
    subscriber: S,
    projector: PJ,
    stop_requested: AtomicBool,
}

impl<PJ, S, R> DefaultProjectorWorker<PJ, S, R> {
    pub fn new(runner: R, subscriber: S, projector: PJ) -> Self {
        Self {
            runner,
            subscriber,
            projector,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<PJ, S, R> ProjectorWorker for DefaultProjectorWorker<PJ, S, R>
where
    PJ: Projector,
    S: Subscriber<EventEnvelope, Selector = EventSelector>,
    S::Consumer: Consumer<EventEnvelope>,
    <S::Consumer as Consumer<EventEnvelope>>::Delivery: Delivery<EventEnvelope>,
    R: ProjectorRunner<Uow = PJ::Uow>,
{
    type Projector = PJ;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), ProjectorWorkerError> {
        let consumer_group = ConsumerGroup::from(<PJ::Spec as ProjectorSpec>::NAME);
        let mut consumer = self
            .subscriber
            .subscribe(&consumer_group, <PJ::Spec as ProjectorSpec>::SUBSCRIPTION)
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            if !<PJ::Spec as ProjectorSpec>::SUBSCRIPTION.matches(delivery.message()) {
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
