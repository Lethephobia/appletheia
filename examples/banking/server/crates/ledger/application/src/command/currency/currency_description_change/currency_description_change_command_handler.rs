use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;

use super::{
    CurrencyDescriptionChangeCommand, CurrencyDescriptionChangeCommandHandlerError,
    CurrencyDescriptionChangeOutput,
};
use crate::authorization::CurrencyDescriptionChangerRelation;

pub struct CurrencyDescriptionChangeCommandHandler<R>
where
    R: Repository<Currency>,
{
    repository: R,
}

impl<R> CurrencyDescriptionChangeCommandHandler<R>
where
    R: Repository<Currency>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyDescriptionChangeCommandHandler<R>
where
    R: Repository<Currency>,
{
    type Command = CurrencyDescriptionChangeCommand;
    type Output = CurrencyDescriptionChangeOutput;
    type Error = CurrencyDescriptionChangeCommandHandlerError;
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
                CurrencyDescriptionChangerRelation::REF,
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
        currency.change_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut currency)
            .await?;
        Ok(CurrencyDescriptionChangeOutput::Changed {
            currency_id: command.currency_id,
        })
    }
}
