use std::marker::PhantomData;

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::request_context::RequestContext;
use appletheia::application::unit_of_work::UnitOfWork;

use super::{OnchainConfigureCommand, OnchainConfigureCommandHandlerError, OnchainConfigureOutput};
use crate::config::OnchainConfigurer;

/// Handles `OnchainConfigureCommand`.
pub struct OnchainConfigureCommandHandler<OCI, U>
where
    OCI: OnchainConfigurer,
    U: UnitOfWork + Sync,
{
    onchain_configurer: OCI,
    _uow: PhantomData<U>,
}

impl<OCI, U> OnchainConfigureCommandHandler<OCI, U>
where
    OCI: OnchainConfigurer,
    U: UnitOfWork + Sync,
{
    pub fn new(onchain_configurer: OCI) -> Self {
        Self {
            onchain_configurer,
            _uow: PhantomData,
        }
    }
}

impl<OCI, U> CommandHandler for OnchainConfigureCommandHandler<OCI, U>
where
    OCI: OnchainConfigurer,
    U: UnitOfWork + Sync,
{
    type Command = OnchainConfigureCommand;
    type Output = OnchainConfigureOutput;
    type ReplayOutput = OnchainConfigureOutput;
    type Error = OnchainConfigureCommandHandlerError;
    type Uow = U;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        _uow: &mut Self::Uow,
        _request_context: &RequestContext,
        _command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        self.onchain_configurer.configure().await?;

        Ok(CommandHandled::same(OnchainConfigureOutput))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::command::CommandHandler;
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use uuid::Uuid;

    use super::{OnchainConfigureCommand, OnchainConfigureCommandHandler, OnchainConfigureOutput};
    use crate::config::{OnchainConfigurer, OnchainConfigurerError};

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestOnchainConfigurer {
        calls: Arc<Mutex<usize>>,
    }

    impl TestOnchainConfigurer {
        fn calls(&self) -> usize {
            *self.calls.lock().expect("lock")
        }
    }

    impl OnchainConfigurer for TestOnchainConfigurer {
        async fn configure(&self) -> Result<(), OnchainConfigurerError> {
            *self.calls.lock().expect("lock") += 1;

            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::System,
        )
        .expect("request context should be valid")
    }

    #[tokio::test]
    async fn handle_configures_onchain() {
        let configurer = TestOnchainConfigurer::default();
        let handler = OnchainConfigureCommandHandler::<_, TestUow>::new(configurer.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(&mut uow, &request_context(), &OnchainConfigureCommand)
            .await
            .expect("command should be handled");

        assert_eq!(configurer.calls(), 1);
        assert_eq!(handled.into_output(), OnchainConfigureOutput);
    }
}
