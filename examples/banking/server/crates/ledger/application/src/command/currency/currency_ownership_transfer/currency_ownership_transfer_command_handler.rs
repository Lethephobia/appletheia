use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyOwnershipTransferResult};

use super::{
    CurrencyOwnershipTransferCommand, CurrencyOwnershipTransferCommandHandlerError,
    CurrencyOwnershipTransferOutput,
};
use crate::authorization::CurrencyOwnershipTransfererRelation;

/// Handles `CurrencyOwnershipTransferCommand`.
pub struct CurrencyOwnershipTransferCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    currency_repository: CR,
}

impl<CR> CurrencyOwnershipTransferCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    pub fn new(currency_repository: CR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CR> CommandHandler for CurrencyOwnershipTransferCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    type Command = CurrencyOwnershipTransferCommand;
    type Output = CurrencyOwnershipTransferOutput;
    type ReplayOutput = CurrencyOwnershipTransferOutput;
    type Error = CurrencyOwnershipTransferCommandHandlerError;
    type Uow = CR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Currency,
            >(
                command.currency_id,
                CurrencyOwnershipTransfererRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyOwnershipTransferCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.transfer_ownership(command.owner)?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyOwnershipTransferResult::Transferred => {
                CurrencyOwnershipTransferOutput::Transferred
            }
            CurrencyOwnershipTransferResult::Rejected { reason } => {
                CurrencyOwnershipTransferOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
