use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;

use super::{
    CurrencyDeactivateCommand, CurrencyDeactivateCommandHandlerError, CurrencyDeactivateOutput,
};
use crate::authorization::CurrencyDeactivatorRelation;

pub struct CurrencyDeactivateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyDeactivateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyDeactivateCommandHandler<R>
where
    R: Repository,
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
        let mut currency = self
            .repository
            .read::<Currency>(uow, command.currency_id)
            .await?;
        currency.deactivate()?;
        self.repository
            .save::<Currency>(uow, request_context, &mut currency)
            .await?;
        Ok(CurrencyDeactivateOutput {
            currency_id: command.currency_id,
        })
    }
}
