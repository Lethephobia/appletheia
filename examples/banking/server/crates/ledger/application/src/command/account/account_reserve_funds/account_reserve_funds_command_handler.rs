use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{
    CommandFailureReaction, CommandFailureReactionError, CommandHandled, CommandHandler,
};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;

use super::{
    AccountReserveFundsCommand, AccountReserveFundsCommandHandlerError, AccountReserveFundsContext,
    AccountReserveFundsOutput,
};
use crate::command::TransferFailCommand;

/// Handles `AccountReserveFundsCommand`.
pub struct AccountReserveFundsCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountReserveFundsCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountReserveFundsCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountReserveFundsCommand;
    type Output = AccountReserveFundsOutput;
    type ReplayOutput = AccountReserveFundsOutput;
    type Error = AccountReserveFundsCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    fn on_failure(
        &self,
        _request_context: &RequestContext,
        command: &Self::Command,
        _error: &Self::Error,
    ) -> Result<CommandFailureReaction, CommandFailureReactionError> {
        match &command.context {
            AccountReserveFundsContext::Transfer { transfer_id } => {
                CommandFailureReaction::with_command(&TransferFailCommand {
                    transfer_id: *transfer_id,
                })
            }
            AccountReserveFundsContext::Direct => Ok(CommandFailureReaction::None),
        }
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut account) = self
            .account_repository
            .find(uow, command.account_id)
            .await?
        else {
            return Err(AccountReserveFundsCommandHandlerError::AccountNotFound);
        };

        account.reserve_funds(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountReserveFundsOutput))
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::command::CommandHandler;
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use banking_ledger_domain::account::{Account, AccountId};
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::transfer::TransferId;

    use super::{
        AccountReserveFundsCommand, AccountReserveFundsCommandHandler,
        AccountReserveFundsCommandHandlerError, AccountReserveFundsContext,
    };
    use crate::command::TransferFailCommand;

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

    struct TestRepository;

    impl Repository<Account> for TestRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: <Account as appletheia::domain::Aggregate>::Id,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            unreachable!("repository is not used in on_failure tests")
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: <Account as appletheia::domain::Aggregate>::Id,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            unreachable!("repository is not used in on_failure tests")
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            unreachable!("repository is not used in on_failure tests")
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            _aggregate: &mut Account,
        ) -> Result<(), RepositoryError<Account>> {
            unreachable!("repository is not used in on_failure tests")
        }
    }

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(uuid::Uuid::now_v7()),
            MessageId::new(),
            ActorRef::System,
            Principal::System,
        )
    }

    #[test]
    fn on_failure_enqueues_transfer_fail_for_transfer_context() {
        let handler = AccountReserveFundsCommandHandler::new(TestRepository);
        let transfer_id = TransferId::new();
        let request_context = request_context();
        let reaction = handler
            .on_failure(
                &request_context,
                &AccountReserveFundsCommand {
                    account_id: AccountId::new(),
                    amount: CurrencyAmount::new(10),
                    context: AccountReserveFundsContext::Transfer { transfer_id },
                },
                &AccountReserveFundsCommandHandlerError::AccountNotFound,
            )
            .expect("reaction should be created");

        let command = reaction
            .into_command_envelopes(&request_context)
            .remove(0)
            .try_into_command::<TransferFailCommand>()
            .expect("command should deserialize");
        assert_eq!(command, TransferFailCommand { transfer_id });
    }
}
