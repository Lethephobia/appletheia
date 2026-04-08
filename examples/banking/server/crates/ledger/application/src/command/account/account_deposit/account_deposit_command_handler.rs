use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{
    CommandFailureReaction, CommandFailureReactionError, CommandHandled, CommandHandler,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::OrganizationOwnerRelationshipProjectorSpec;
use banking_ledger_domain::account::Account;

use super::{
    AccountDepositCommand, AccountDepositCommandHandlerError, AccountDepositContext,
    AccountDepositOutput,
};
use crate::authorization::AccountDepositorRelation;
use crate::command::{
    AccountReleaseReservedFundsCommand, AccountReleaseReservedFundsContext,
    CurrencyDefinitionDecreaseSupplyCommand, CurrencyDefinitionDecreaseSupplyContext,
};
use crate::projection::AccountOwnerRelationshipProjectorSpec;

/// Handles `AccountDepositCommand`.
pub struct AccountDepositCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountDepositCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountDepositCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountDepositCommand;
    type Output = AccountDepositOutput;
    type ReplayOutput = AccountDepositOutput;
    type Error = AccountDepositCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Account>(
                    command.account_id,
                    AccountDepositorRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    AccountOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    fn on_failure(
        &self,
        _request_context: &RequestContext,
        command: &Self::Command,
        _error: &Self::Error,
    ) -> Result<CommandFailureReaction, CommandFailureReactionError> {
        match &command.context {
            AccountDepositContext::Transfer {
                transfer_id,
                from_account_id,
            } => CommandFailureReaction::with_command(&AccountReleaseReservedFundsCommand {
                account_id: *from_account_id,
                amount: command.amount,
                context: AccountReleaseReservedFundsContext::Transfer {
                    transfer_id: *transfer_id,
                },
            }),
            AccountDepositContext::Issuance {
                currency_issuance_id,
                currency_definition_id,
            } => CommandFailureReaction::with_command(&CurrencyDefinitionDecreaseSupplyCommand {
                currency_definition_id: *currency_definition_id,
                amount: command.amount,
                context: CurrencyDefinitionDecreaseSupplyContext::Issuance {
                    currency_issuance_id: *currency_issuance_id,
                },
            }),
            AccountDepositContext::Direct => Ok(CommandFailureReaction::None),
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
            return Err(AccountDepositCommandHandlerError::AccountNotFound);
        };

        account.deposit(command.amount)?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountDepositOutput))
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
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
    use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
    use banking_ledger_domain::transfer::TransferId;

    use super::{
        AccountDepositCommand, AccountDepositCommandHandler, AccountDepositCommandHandlerError,
        AccountDepositContext,
    };
    use crate::command::{
        AccountReleaseReservedFundsCommand, AccountReleaseReservedFundsContext,
        CurrencyDefinitionDecreaseSupplyCommand, CurrencyDefinitionDecreaseSupplyContext,
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
    fn on_failure_enqueues_release_reserved_funds_for_transfer_context() {
        let handler = AccountDepositCommandHandler::new(TestRepository);
        let transfer_id = TransferId::new();
        let from_account_id = AccountId::new();
        let to_account_id = AccountId::new();
        let amount = CurrencyAmount::new(10);
        let request_context = request_context();

        let reaction = handler
            .on_failure(
                &request_context,
                &AccountDepositCommand {
                    account_id: to_account_id,
                    amount,
                    context: AccountDepositContext::Transfer {
                        transfer_id,
                        from_account_id,
                    },
                },
                &AccountDepositCommandHandlerError::AccountNotFound,
            )
            .expect("reaction should be created");

        let command = reaction
            .into_command_envelopes(&request_context)
            .remove(0)
            .try_into_command::<AccountReleaseReservedFundsCommand>()
            .expect("command should deserialize");
        assert_eq!(
            command,
            AccountReleaseReservedFundsCommand {
                account_id: from_account_id,
                amount,
                context: AccountReleaseReservedFundsContext::Transfer { transfer_id },
            }
        );
    }

    #[test]
    fn on_failure_enqueues_supply_decrease_for_issuance_context() {
        let handler = AccountDepositCommandHandler::new(TestRepository);
        let currency_issuance_id = CurrencyIssuanceId::new();
        let currency_definition_id = CurrencyDefinitionId::new();
        let amount = CurrencyAmount::new(10);
        let request_context = request_context();

        let reaction = handler
            .on_failure(
                &request_context,
                &AccountDepositCommand {
                    account_id: AccountId::new(),
                    amount,
                    context: AccountDepositContext::Issuance {
                        currency_issuance_id,
                        currency_definition_id,
                    },
                },
                &AccountDepositCommandHandlerError::AccountNotFound,
            )
            .expect("reaction should be created");

        let command = reaction
            .into_command_envelopes(&request_context)
            .remove(0)
            .try_into_command::<CurrencyDefinitionDecreaseSupplyCommand>()
            .expect("command should deserialize");
        assert_eq!(
            command,
            CurrencyDefinitionDecreaseSupplyCommand {
                currency_definition_id,
                amount,
                context: CurrencyDefinitionDecreaseSupplyContext::Issuance {
                    currency_issuance_id,
                },
            }
        );
    }
}
