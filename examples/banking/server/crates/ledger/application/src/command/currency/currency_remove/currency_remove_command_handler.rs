use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyRemoveResult};

use super::{CurrencyRemoveCommand, CurrencyRemoveCommandHandlerError, CurrencyRemoveOutput};
use crate::authorization::CurrencyRemoverRelation;

/// Handles `CurrencyRemoveCommand`.
pub struct CurrencyRemoveCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencyRemoveCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyRemoveCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencyRemoveCommand;
    type Output = CurrencyRemoveOutput;
    type ReplayOutput = CurrencyRemoveOutput;
    type Error = CurrencyRemoveCommandHandlerError;
    type Uow = CDR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Currency,
            >(
                command.currency_id,
                CurrencyRemoverRelation::REF,
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
            return Err(CurrencyRemoveCommandHandlerError::CurrencyNotFound);
        };

        let result = currency.remove()?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyRemoveResult::Removed => CurrencyRemoveOutput::Removed,
            CurrencyRemoveResult::Rejected { reason } => CurrencyRemoveOutput::Rejected { reason },
        };

        Ok(CommandHandled::same(output))
    }
}
