use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::{Account, AccountOwnershipTransferResult};

use super::{
    AccountOwnershipTransferCommand, AccountOwnershipTransferCommandHandlerError,
    AccountOwnershipTransferOutput,
};
use crate::authorization::AccountOwnershipTransfererRelation;

/// Handles `AccountOwnershipTransferCommand`.
pub struct AccountOwnershipTransferCommandHandler<AR>
where
    AR: Repository<Account>,
{
    account_repository: AR,
}

impl<AR> AccountOwnershipTransferCommandHandler<AR>
where
    AR: Repository<Account>,
{
    pub fn new(account_repository: AR) -> Self {
        Self { account_repository }
    }
}

impl<AR> CommandHandler for AccountOwnershipTransferCommandHandler<AR>
where
    AR: Repository<Account>,
{
    type Command = AccountOwnershipTransferCommand;
    type Output = AccountOwnershipTransferOutput;
    type Error = AccountOwnershipTransferCommandHandlerError;
    type Uow = AR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountOwnershipTransfererRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;

        let result = account.transfer_ownership(command.owner)?;

        self.account_repository
            .save(uow, request_context, &mut account)
            .await?;

        let output = match result {
            AccountOwnershipTransferResult::Transferred => {
                AccountOwnershipTransferOutput::Transferred
            }
            AccountOwnershipTransferResult::Rejected { reason } => {
                AccountOwnershipTransferOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
