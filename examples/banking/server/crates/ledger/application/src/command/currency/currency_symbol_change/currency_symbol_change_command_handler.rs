use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_ledger_domain::currency::{
    Currency, CurrencyState, CurrencySymbol, CurrencySymbolChangeRejectionReason,
    CurrencySymbolChangeResult,
};

use super::{
    CurrencySymbolChangeCommand, CurrencySymbolChangeCommandHandlerError,
    CurrencySymbolChangeOutput,
};
use crate::authorization::CurrencyUpdaterRelation;

/// Handles `CurrencySymbolChangeCommand`.
pub struct CurrencySymbolChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    currency_repository: CR,
}

impl<CR> CurrencySymbolChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    pub fn new(currency_repository: CR) -> Self {
        Self {
            currency_repository,
        }
    }

    fn symbol_unique_value(
        symbol: &CurrencySymbol,
    ) -> Result<UniqueValue, CurrencySymbolChangeCommandHandlerError> {
        Ok(UniqueValue::from_strings([symbol.as_ref()])?)
    }
}

impl<CR> CommandHandler for CurrencySymbolChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    type Command = CurrencySymbolChangeCommand;
    type Output = CurrencySymbolChangeOutput;
    type Error = CurrencySymbolChangeCommandHandlerError;
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
                CurrencyUpdaterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut currency = self
            .currency_repository
            .read(uow, command.currency_id)
            .await?;

        let unique_value = Self::symbol_unique_value(&command.symbol)?;
        if self
            .currency_repository
            .find_by_unique_value(uow, CurrencyState::SYMBOL_KEY, &unique_value)
            .await?
            .is_some_and(|existing| existing.aggregate_id() != command.currency_id)
        {
            let reason = CurrencySymbolChangeRejectionReason::AlreadyTaken;
            currency.reject_change_symbol(command.symbol.clone(), reason)?;

            self.currency_repository
                .save(uow, request_context, &mut currency)
                .await?;

            return Ok(CurrencySymbolChangeOutput::Rejected { reason });
        }

        let result = currency.change_symbol(command.symbol.clone())?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencySymbolChangeResult::Changed => CurrencySymbolChangeOutput::Changed,
            CurrencySymbolChangeResult::Rejected { reason } => {
                CurrencySymbolChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
