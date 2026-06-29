use std::marker::PhantomData;

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::request_context::RequestContext;
use appletheia::application::unit_of_work::UnitOfWork;

use super::{
    BankingLedgerConfigConfigureCommand, BankingLedgerConfigConfigureCommandHandlerError,
    BankingLedgerConfigConfigureOutput,
};
use crate::banking_ledger::BankingLedgerConfigConfigurer;

/// Handles `BankingLedgerConfigConfigureCommand`.
pub struct BankingLedgerConfigConfigureCommandHandler<BLCI, U>
where
    BLCI: BankingLedgerConfigConfigurer,
    U: UnitOfWork + Sync,
{
    banking_ledger_config_configurer: BLCI,
    _uow: PhantomData<U>,
}

impl<BLCI, U> BankingLedgerConfigConfigureCommandHandler<BLCI, U>
where
    BLCI: BankingLedgerConfigConfigurer,
    U: UnitOfWork + Sync,
{
    pub fn new(banking_ledger_config_configurer: BLCI) -> Self {
        Self {
            banking_ledger_config_configurer,
            _uow: PhantomData,
        }
    }
}

impl<BLCI, U> CommandHandler for BankingLedgerConfigConfigureCommandHandler<BLCI, U>
where
    BLCI: BankingLedgerConfigConfigurer,
    U: UnitOfWork + Sync,
{
    type Command = BankingLedgerConfigConfigureCommand;
    type Output = BankingLedgerConfigConfigureOutput;
    type ReplayOutput = BankingLedgerConfigConfigureOutput;
    type Error = BankingLedgerConfigConfigureCommandHandlerError;
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
        self.banking_ledger_config_configurer.configure().await?;

        Ok(CommandHandled::same(BankingLedgerConfigConfigureOutput))
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

    use super::{
        BankingLedgerConfigConfigureCommand, BankingLedgerConfigConfigureCommandHandler,
        BankingLedgerConfigConfigureOutput,
    };
    use crate::banking_ledger::{
        BankingLedgerConfigConfigurer, BankingLedgerConfigConfigurerError,
    };

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
    struct TestBankingLedgerConfigConfigurer {
        calls: Arc<Mutex<usize>>,
    }

    impl TestBankingLedgerConfigConfigurer {
        fn calls(&self) -> usize {
            *self.calls.lock().expect("lock")
        }
    }

    impl BankingLedgerConfigConfigurer for TestBankingLedgerConfigConfigurer {
        async fn configure(&self) -> Result<(), BankingLedgerConfigConfigurerError> {
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
    async fn handle_configures_banking_ledger_config() {
        let configurer = TestBankingLedgerConfigConfigurer::default();
        let handler =
            BankingLedgerConfigConfigureCommandHandler::<_, TestUow>::new(configurer.clone());
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &BankingLedgerConfigConfigureCommand,
            )
            .await
            .expect("command should be handled");

        assert_eq!(configurer.calls(), 1);
        assert_eq!(handled.into_output(), BankingLedgerConfigConfigureOutput);
    }
}
