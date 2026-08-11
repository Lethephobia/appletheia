use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyNameChangeResult};

use super::{
    CurrencyNameChangeCommand, CurrencyNameChangeCommandHandlerError, CurrencyNameChangeOutput,
};
use crate::authorization::CurrencyUpdaterRelation;

/// Handles `CurrencyNameChangeCommand`.
pub struct CurrencyNameChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    currency_repository: CR,
}

impl<CR> CurrencyNameChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    pub fn new(currency_repository: CR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CR> CommandHandler for CurrencyNameChangeCommandHandler<CR>
where
    CR: Repository<Currency>,
{
    type Command = CurrencyNameChangeCommand;
    type Output = CurrencyNameChangeOutput;
    type Error = CurrencyNameChangeCommandHandlerError;
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

        let result = currency.change_name(command.name.clone())?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyNameChangeResult::Changed => CurrencyNameChangeOutput::Changed,
            CurrencyNameChangeResult::Rejected { reason } => {
                CurrencyNameChangeOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
