use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::RoleAssigneeRelationshipProjectorSpec;
use banking_ledger_domain::account::Account;

use super::{AccountFreezeCommand, AccountFreezeCommandHandlerError, AccountFreezeOutput};
use crate::authorization::AccountFreezerRelation;
use crate::projection::AccountStatusManagerRelationshipProjectorSpec;

/// Handles `AccountFreezeCommand`.
pub struct AccountFreezeCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountFreezeCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountFreezeCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountFreezeCommand;
    type Output = AccountFreezeOutput;
    type ReplayOutput = AccountFreezeOutput;
    type Error = AccountFreezeCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<Account>(command.account_id),
                    relation: AccountFreezerRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    RoleAssigneeRelationshipProjectorSpec::DESCRIPTOR,
                    AccountStatusManagerRelationshipProjectorSpec::DESCRIPTOR,
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
        let Some(mut account) = self
            .account_repository
            .find(uow, command.account_id)
            .await?
        else {
            return Err(AccountFreezeCommandHandlerError::AccountNotFound);
        };

        account.freeze()?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountFreezeOutput))
    }
}
