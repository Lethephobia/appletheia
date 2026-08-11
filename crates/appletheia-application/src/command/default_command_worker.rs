use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::Retryability;
use crate::command::{
    Command, CommandDispatcher, CommandEnvelope, CommandEnvelopeError, CommandHandler,
    CommandSelector, CommandWorker,
};
use crate::messaging::Subscription;
use crate::request_context::{ActorRef, Principal, RequestContext};
use crate::{Consumer, ConsumerGroup, Delivery, Subscriber};

use super::CommandWorkerError;

pub struct DefaultCommandWorker<H, D, S>
where
    H: CommandHandler,
    H::Command: Command,
    D: CommandDispatcher<Uow = H::Uow>,
    S: Subscriber<CommandEnvelope, Selector = CommandSelector>,
    S::Consumer: Consumer<CommandEnvelope>,
    <S::Consumer as Consumer<CommandEnvelope>>::Delivery: Delivery<CommandEnvelope>,
{
    dispatcher: D,
    handler: H,
    subscriber: S,
    consumer_group: ConsumerGroup,
    stop_requested: AtomicBool,
}

impl<H, D, S> DefaultCommandWorker<H, D, S>
where
    H: CommandHandler,
    H::Command: Command,
    D: CommandDispatcher<Uow = H::Uow>,
    S: Subscriber<CommandEnvelope, Selector = CommandSelector>,
    S::Consumer: Consumer<CommandEnvelope>,
    <S::Consumer as Consumer<CommandEnvelope>>::Delivery: Delivery<CommandEnvelope>,
{
    pub fn new(dispatcher: D, handler: H, subscriber: S, consumer_group: ConsumerGroup) -> Self {
        Self {
            dispatcher,
            handler,
            subscriber,
            consumer_group,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl<H, D, S> CommandWorker for DefaultCommandWorker<H, D, S>
where
    H: CommandHandler,
    H::Command: Command,
    D: CommandDispatcher<Uow = H::Uow>,
    S: Subscriber<CommandEnvelope, Selector = CommandSelector>,
    S::Consumer: Consumer<CommandEnvelope>,
    <S::Consumer as Consumer<CommandEnvelope>>::Delivery: Delivery<CommandEnvelope>,
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
            .subscriber
            .subscribe(&self.consumer_group, Subscription::AnyOf(&selectors))
            .await?;

        while !self.is_stop_requested() {
            let mut delivery = consumer.next().await?;

            let decoded_command = match delivery.message().try_into_command::<H::Command>() {
                Ok(command) => Some(command),
                Err(CommandEnvelopeError::CommandNameMismatch { .. }) => {
                    delivery.ack().await?;
                    None
                }
                Err(error) => {
                    delivery.nack().await?;
                    return Err(error.into());
                }
            };

            if let Some(command) = decoded_command {
                let envelope = delivery.message();
                let request_context = RequestContext {
                    correlation_id: envelope.correlation_id,
                    message_id: envelope.message_id,
                    actor: ActorRef::System,
                    principal: Principal::System,
                };

                let result = self
                    .dispatcher
                    .dispatch(
                        &self.handler,
                        &request_context,
                        command,
                        envelope.options.clone(),
                    )
                    .await;

                match result {
                    Ok(_) => delivery.ack().await?,
                    Err(error) => {
                        if error.is_retryable() {
                            delivery.nack().await?;
                        } else {
                            delivery.ack().await?;
                        }
                        return Err(CommandWorkerError::Dispatch(Box::new(error)));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::DefaultCommandWorker;
    use crate::Retryability;
    use crate::authorization::AuthorizationPlan;
    use crate::command::CommandEnvelope;
    use crate::command::{
        Command, CommandDispatchResult, CommandDispatcher, CommandDispatcherError, CommandHandler,
        CommandName, CommandOptions, CommandOutput, CommandReplayOutput, CommandWorker,
    };
    use crate::messaging::{
        Consumer, ConsumerError, ConsumerGroup, Delivery, Subscriber, SubscriberError, Subscription,
    };
    use crate::request_context::{CausationId, CorrelationId, MessageId, RequestContext};
    use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TestCommand {}

    impl Command for TestCommand {
        const NAME: CommandName = CommandName::new("test");
    }

    #[derive(Debug, thiserror::Error)]
    #[error("test handler error")]
    struct TestHandlerError {
        retryable: bool,
    }

    impl Retryability for TestHandlerError {
        fn is_retryable(&self) -> bool {
            self.retryable
        }
    }

    struct TestHandler {
        retryable: bool,
    }

    #[derive(Deserialize, Serialize)]
    struct TestOutput;

    impl CommandOutput for TestOutput {
        type ReplayOutput = Self;

        fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
            CommandReplayOutput::Borrowed(self)
        }
    }

    impl CommandHandler for TestHandler {
        type Command = TestCommand;
        type Output = TestOutput;
        type Error = TestHandlerError;
        type Uow = TestUow;

        fn authorization_plan(
            &self,
            _command: &Self::Command,
        ) -> Result<AuthorizationPlan, Self::Error> {
            Err(TestHandlerError {
                retryable: self.retryable,
            })
        }

        async fn handle(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _command: &Self::Command,
        ) -> Result<Self::Output, Self::Error> {
            unreachable!("test dispatcher does not call the handler")
        }
    }

    struct TestDispatcher;

    impl CommandDispatcher for TestDispatcher {
        type Uow = TestUow;

        async fn dispatch<H>(
            &self,
            handler: &H,
            _request_context: &RequestContext,
            command: H::Command,
            _options: CommandOptions,
        ) -> Result<
            CommandDispatchResult<H::Output, <H::Output as CommandOutput>::ReplayOutput>,
            CommandDispatcherError<H::Error>,
        >
        where
            H: CommandHandler<Uow = Self::Uow>,
            H::Command: Command,
        {
            match handler.authorization_plan(&command) {
                Ok(_) => unreachable!("test handler should return an authorization error"),
                Err(error) => Err(CommandDispatcherError::Handler(error)),
            }
        }
    }

    #[derive(Default)]
    struct DeliveryState {
        acknowledgements: AtomicUsize,
        negative_acknowledgements: AtomicUsize,
    }

    struct TestDelivery {
        envelope: CommandEnvelope,
        state: Arc<DeliveryState>,
    }

    impl Delivery<CommandEnvelope> for TestDelivery {
        fn message(&self) -> &CommandEnvelope {
            &self.envelope
        }

        async fn ack(&mut self) -> Result<(), ConsumerError> {
            self.state.acknowledgements.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn nack(&mut self) -> Result<(), ConsumerError> {
            self.state
                .negative_acknowledgements
                .fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct TestConsumer {
        delivery: Option<TestDelivery>,
    }

    impl Consumer<CommandEnvelope> for TestConsumer {
        type Delivery = TestDelivery;

        async fn next(&mut self) -> Result<Self::Delivery, ConsumerError> {
            Ok(self.delivery.take().expect("test delivery should exist"))
        }
    }

    struct TestSubscriber {
        envelope: CommandEnvelope,
        state: Arc<DeliveryState>,
    }

    impl Subscriber<CommandEnvelope> for TestSubscriber {
        type Consumer = TestConsumer;
        type Selector = crate::command::CommandSelector;

        async fn subscribe(
            &self,
            _consumer_group: &ConsumerGroup,
            _subscription: Subscription<'_, Self::Selector>,
        ) -> Result<Self::Consumer, SubscriberError> {
            Ok(TestConsumer {
                delivery: Some(TestDelivery {
                    envelope: self.envelope.clone(),
                    state: Arc::clone(&self.state),
                }),
            })
        }
    }

    fn build_worker(
        retryable: bool,
        state: Arc<DeliveryState>,
    ) -> DefaultCommandWorker<TestHandler, TestDispatcher, TestSubscriber> {
        let causation_message_id = MessageId::new();
        let envelope = CommandEnvelope::new(
            &TestCommand {},
            CorrelationId::from(Uuid::now_v7()),
            CausationId::from(causation_message_id),
            CommandOptions::default(),
        )
        .expect("command envelope should be valid");
        let consumer_group =
            ConsumerGroup::new("test".to_owned()).expect("consumer group should be valid");

        DefaultCommandWorker::new(
            TestDispatcher,
            TestHandler { retryable },
            TestSubscriber { envelope, state },
            consumer_group,
        )
    }

    #[tokio::test]
    async fn run_forever_nacks_retryable_dispatch_failure() {
        let state = Arc::new(DeliveryState::default());
        let mut command_worker = build_worker(true, Arc::clone(&state));

        command_worker
            .run_forever()
            .await
            .expect_err("dispatch failure should be returned");

        assert_eq!(state.acknowledgements.load(Ordering::SeqCst), 0);
        assert_eq!(state.negative_acknowledgements.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn run_forever_acks_non_retryable_dispatch_failure() {
        let state = Arc::new(DeliveryState::default());
        let mut command_worker = build_worker(false, Arc::clone(&state));

        command_worker
            .run_forever()
            .await
            .expect_err("dispatch failure should be returned");

        assert_eq!(state.acknowledgements.load(Ordering::SeqCst), 1);
        assert_eq!(state.negative_acknowledgements.load(Ordering::SeqCst), 0);
    }
}
