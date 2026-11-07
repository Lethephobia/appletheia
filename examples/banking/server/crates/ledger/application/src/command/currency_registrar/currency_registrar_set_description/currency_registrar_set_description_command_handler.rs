use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;

use crate::authorization::CurrencyRegistrarMemberRelation;

use super::{
    CurrencyRegistrarSetDescriptionCommand, CurrencyRegistrarSetDescriptionCommandHandlerError,
    CurrencyRegistrarSetDescriptionOutput,
};

pub struct CurrencyRegistrarSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarSetDescriptionCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarSetDescriptionCommand;
    type Output = CurrencyRegistrarSetDescriptionOutput;
    type Error = CurrencyRegistrarSetDescriptionCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                CurrencyRegistrar,
            >(
                command.currency_registrar_id,
                CurrencyRegistrarMemberRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut registrar = self
            .repository
            .read::<CurrencyRegistrar>(uow, command.currency_registrar_id)
            .await?;
        registrar.set_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut registrar)
            .await?;
        Ok(CurrencyRegistrarSetDescriptionOutput {})
    }
}
