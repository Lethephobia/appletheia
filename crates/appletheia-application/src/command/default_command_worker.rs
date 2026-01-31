use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::command::{Command, CommandDispatcher, CommandHandler, CommandSelector, CommandWorker};
use crate::messaging::Subscription;
use crate::outbox::command::{CommandEnvelope, CommandEnvelopeError};
use crate::request_context::RequestContext;
use crate::{Consumer, ConsumerGroup, Delivery, Topic};

use super::CommandWorkerError;

pub struct DefaultCommandWorker<H, D, T> {
    dispatcher: D,
    handler: H,
    topic: T,
    consumer_group: ConsumerGroup,
    stop_requested: AtomicBool,
}

impl<H, D, T> DefaultCommandWorker<H, D, T> {
    pub fn new(dispatcher: D, handler: H, topic: T, consumer_group: ConsumerGroup) -> Self {
        Self {
            dispatcher,
            handler,
            topic,
            consumer_group,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<H, D, T> CommandWorker for DefaultCommandWorker<H, D, T>
where
    H: CommandHandler,
    H::Command: Command,
    D: CommandDispatcher<Uow = H::Uow>,
    T: Topic<CommandEnvelope, Selector = CommandSelector>,
    T::Consumer: Consumer<CommandEnvelope>,
    <T::Consumer as Consumer<CommandEnvelope>>::Delivery: Delivery<CommandEnvelope>,
{
    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&mut self) -> Result<(), CommandWorkerError> {
        let selectors = [CommandSelector::new(H::Command::NAME)];

        let mut consumer = self
            .topic
            .subscribe(&self.consumer_group, Subscription::Only(&selectors))
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;
            let envelope = delivery.message();

            let command = match envelope.try_into_command::<H::Command>() {
                Ok(command) => command,
                Err(CommandEnvelopeError::CommandNameMismatch { .. }) => {
                    delivery.ack().await?;
                    continue;
                }
                Err(error) => {
                    delivery.nack().await?;
                    return Err(error.into());
                }
            };

            let request_context = RequestContext {
                correlation_id: envelope.correlation_id,
                message_id: envelope.message_id,
            };

            let result = self
                .dispatcher
                .dispatch(&self.handler, &request_context, command)
                .await;

            match result {
                Ok(_) => delivery.ack().await?,
                Err(error) => {
                    delivery.nack().await?;
                    return Err(CommandWorkerError::Dispatch(Box::new(error)));
                }
            }
        }

        Ok(())
    }
}
