use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyLifecycleResult};

use super::{CurrencyActivateCommand, CurrencyActivateCommandHandlerError, CurrencyActivateOutput};
use crate::authorization::CurrencyActivatorRelation;

pub struct CurrencyActivateCommandHandler<R>
where
    R: Repository<Currency>,
{
    repository: R,
}

impl<R> CurrencyActivateCommandHandler<R>
where
    R: Repository<Currency>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyActivateCommandHandler<R>
where
    R: Repository<Currency>,
{
    type Command = CurrencyActivateCommand;
    type Output = CurrencyActivateOutput;
    type Error = CurrencyActivateCommandHandlerError;
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
                CurrencyActivatorRelation::REF,
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
        let result = currency.activate()?;
        self.repository
            .save(uow, request_context, &mut currency)
            .await?;
        Ok(match result {
            CurrencyLifecycleResult::Changed => CurrencyActivateOutput::Activated {
                currency_id: command.currency_id,
            },
            CurrencyLifecycleResult::Rejected { reason } => CurrencyActivateOutput::Rejected {
                currency_id: command.currency_id,
                reason,
            },
        })
    }
}
