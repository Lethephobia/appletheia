use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyLifecycleResult};

use super::{
    CurrencyDeactivateCommand, CurrencyDeactivateCommandHandlerError, CurrencyDeactivateOutput,
};
use crate::authorization::CurrencyDeactivatorRelation;

pub struct CurrencyDeactivateCommandHandler<R>
where
    R: Repository<Currency>,
{
    repository: R,
}

impl<R> CurrencyDeactivateCommandHandler<R>
where
    R: Repository<Currency>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyDeactivateCommandHandler<R>
where
    R: Repository<Currency>,
{
    type Command = CurrencyDeactivateCommand;
    type Output = CurrencyDeactivateOutput;
    type Error = CurrencyDeactivateCommandHandlerError;
    type Uow = R::Uow;

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
        let mut currency = self.repository.read(uow, command.currency_id).await?;
        let result = currency.deactivate()?;
        self.repository
            .save(uow, request_context, &mut currency)
            .await?;
        Ok(match result {
            CurrencyLifecycleResult::Changed => CurrencyDeactivateOutput::Deactivated {
                currency_id: command.currency_id,
            },
            CurrencyLifecycleResult::Rejected { reason } => CurrencyDeactivateOutput::Rejected {
                currency_id: command.currency_id,
                reason,
            },
        })
    }
}
