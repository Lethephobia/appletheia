use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::{RoleAssigneeRelation, UserOwnerRelation};
use banking_iam_application::{
    RoleAssigneeRelationshipProjectorSpec, UserOwnerRelationshipProjectorSpec,
};
use banking_iam_domain::{Role, RoleId, User};
use banking_ledger_domain::account::Account;

use super::{AccountOpenCommand, AccountOpenCommandHandlerError, AccountOpenOutput};

/// Handles `AccountOpenCommand`.
pub struct AccountOpenCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountOpenCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountOpenCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountOpenCommand;
    type Output = AccountOpenOutput;
    type ReplayOutput = AccountOpenOutput;
    type Error = AccountOpenCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Any(vec![
                    RelationshipRequirement::Check {
                        aggregate: AggregateRef::from_id::<Role>(RoleId::admin()),
                        relation: RoleAssigneeRelation::NAME,
                    },
                    RelationshipRequirement::Check {
                        aggregate: AggregateRef::from_id::<User>(command.user_id),
                        relation: UserOwnerRelation::NAME,
                    },
                ]),
                projector_dependencies: ProjectorDependencies::Some(&[
                    RoleAssigneeRelationshipProjectorSpec::DESCRIPTOR,
                    UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut account = Account::default();
        account.open(command.user_id, command.currency_definition_id)?;

        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let account_id = account
            .aggregate_id()
            .ok_or(AccountOpenCommandHandlerError::MissingAccountId)?;

        Ok(CommandHandled::same(AccountOpenOutput::new(account_id)))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        ActorRef, CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_application::authorization::{RoleAssigneeRelation, UserOwnerRelation};
    use banking_iam_application::{
        RoleAssigneeRelationshipProjectorSpec, UserOwnerRelationshipProjectorSpec,
    };
    use banking_iam_domain::{Role, RoleId, User, UserId};
    use banking_ledger_domain::account::{Account, AccountId};
    use banking_ledger_domain::currency_definition::CurrencyDefinitionId;
    use uuid::Uuid;

    use super::{AccountOpenCommand, AccountOpenCommandHandler, AccountOpenOutput};

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
    struct TestAccountRepository {
        account: Arc<Mutex<Option<Account>>>,
    }

    impl Repository<Account> for TestAccountRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.account.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: AccountId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(self.account.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<Account>, RepositoryError<Account>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Account,
        ) -> Result<(), RepositoryError<Account>> {
            *self.account.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    fn request_context() -> RequestContext {
        let subject = AggregateRef::from_id::<User>(UserId::new());

        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            ActorRef::Subject {
                subject: subject.clone(),
            },
            Principal::Authenticated { subject },
        )
    }

    #[test]
    fn authorization_plan_allows_system_admin_or_target_user_owner() {
        let handler = AccountOpenCommandHandler::new(TestAccountRepository::default());
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&AccountOpenCommand {
                user_id,
                currency_definition_id: CurrencyDefinitionId::new(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::System,
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::Any(vec![
                        RelationshipRequirement::Check {
                            aggregate: AggregateRef::from_id::<Role>(RoleId::admin()),
                            relation: RoleAssigneeRelation::NAME,
                        },
                        RelationshipRequirement::Check {
                            aggregate: AggregateRef::from_id::<User>(user_id),
                            relation: UserOwnerRelation::NAME,
                        },
                    ]),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        RoleAssigneeRelationshipProjectorSpec::DESCRIPTOR,
                        UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_opens_account_for_specified_user() {
        let repository = TestAccountRepository::default();
        let handler = AccountOpenCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let user_id = UserId::new();

        let handled = handler
            .handle(
                &mut uow,
                &request_context(),
                &AccountOpenCommand {
                    user_id,
                    currency_definition_id: CurrencyDefinitionId::new(),
                },
            )
            .await
            .expect("command should succeed");

        let saved = repository
            .account
            .lock()
            .expect("lock")
            .clone()
            .expect("account should be saved");
        let account_id = saved.aggregate_id().expect("account id should exist");
        assert_eq!(saved.user_id().expect("user id should exist"), &user_id);

        assert_eq!(handled.into_output(), AccountOpenOutput::new(account_id));
    }
}
