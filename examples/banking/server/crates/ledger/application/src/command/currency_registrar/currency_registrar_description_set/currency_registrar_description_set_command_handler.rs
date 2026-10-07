use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;

use crate::authorization::CurrencyRegistrarMemberRelation;

use super::{
    CurrencyRegistrarDescriptionSetCommand, CurrencyRegistrarDescriptionSetCommandHandlerError,
    CurrencyRegistrarDescriptionSetOutput,
};

pub struct CurrencyRegistrarDescriptionSetCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarDescriptionSetCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarDescriptionSetCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarDescriptionSetCommand;
    type Output = CurrencyRegistrarDescriptionSetOutput;
    type Error = CurrencyRegistrarDescriptionSetCommandHandlerError;
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
            .save::<CurrencyRegistrar>(uow, request_context, &mut registrar)
            .await?;
        Ok(CurrencyRegistrarDescriptionSetOutput {})
    }
}
