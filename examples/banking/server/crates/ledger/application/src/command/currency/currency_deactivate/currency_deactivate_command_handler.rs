use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyDeactivateResult};

use super::{
    CurrencyDeactivateCommand, CurrencyDeactivateCommandHandlerError, CurrencyDeactivateOutput,
};
use crate::authorization::CurrencyDeactivatorRelation;

/// Handles `CurrencyDeactivateCommand`.
pub struct CurrencyDeactivateCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    currency_repository: CDR,
}

impl<CDR> CurrencyDeactivateCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    pub fn new(currency_repository: CDR) -> Self {
        Self {
            currency_repository,
        }
    }
}

impl<CDR> CommandHandler for CurrencyDeactivateCommandHandler<CDR>
where
    CDR: Repository<Currency>,
{
    type Command = CurrencyDeactivateCommand;
    type Output = CurrencyDeactivateOutput;
    type Error = CurrencyDeactivateCommandHandlerError;
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
                CurrencyDeactivatorRelation::REF,
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

        let result = currency.deactivate()?;

        self.currency_repository
            .save(uow, request_context, &mut currency)
            .await?;

        let output = match result {
            CurrencyDeactivateResult::Deactivated => CurrencyDeactivateOutput::Deactivated,
            CurrencyDeactivateResult::Rejected { reason } => {
                CurrencyDeactivateOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
