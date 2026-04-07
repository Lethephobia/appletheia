use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_application::OrganizationOwnerRelationshipProjectorSpec;
use banking_ledger_domain::account::Account;

use super::{AccountCloseCommand, AccountCloseCommandHandlerError, AccountCloseOutput};
use crate::authorization::AccountCloserRelation;
use crate::projection::AccountOwnerRelationshipProjectorSpec;

/// Handles `AccountCloseCommand`.
pub struct AccountCloseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountCloseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountCloseCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountCloseCommand;
    type Output = AccountCloseOutput;
    type ReplayOutput = AccountCloseOutput;
    type Error = AccountCloseCommandHandlerError;
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
                    AccountCloserRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    AccountOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
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
            return Err(AccountCloseCommandHandlerError::AccountNotFound);
        };

        account.close()?;
        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        Ok(CommandHandled::same(AccountCloseOutput))
    }
}
