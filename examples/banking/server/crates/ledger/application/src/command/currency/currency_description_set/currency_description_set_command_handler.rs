use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;

use super::{
    CurrencyDescriptionSetCommand, CurrencyDescriptionSetCommandHandlerError,
    CurrencyDescriptionSetOutput,
};
use crate::authorization::CurrencyDescriptionSetterRelation;

pub struct CurrencyDescriptionSetCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyDescriptionSetCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyDescriptionSetCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyDescriptionSetCommand;
    type Output = CurrencyDescriptionSetOutput;
    type Error = CurrencyDescriptionSetCommandHandlerError;
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
                CurrencyDescriptionSetterRelation::REF,
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
        currency.set_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut currency)
            .await?;
        Ok(CurrencyDescriptionSetOutput {
            currency_id: command.currency_id,
        })
    }
}
