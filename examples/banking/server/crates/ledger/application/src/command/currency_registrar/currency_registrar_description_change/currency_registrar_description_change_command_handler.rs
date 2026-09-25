use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_registrar::CurrencyRegistrar;

use crate::authorization::CurrencyRegistrarMemberRelation;

use super::{
    CurrencyRegistrarDescriptionChangeCommand,
    CurrencyRegistrarDescriptionChangeCommandHandlerError,
    CurrencyRegistrarDescriptionChangeOutput,
};

pub struct CurrencyRegistrarDescriptionChangeCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    repository: R,
}

impl<R> CurrencyRegistrarDescriptionChangeCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarDescriptionChangeCommandHandler<R>
where
    R: Repository<CurrencyRegistrar>,
{
    type Command = CurrencyRegistrarDescriptionChangeCommand;
    type Output = CurrencyRegistrarDescriptionChangeOutput;
    type Error = CurrencyRegistrarDescriptionChangeCommandHandlerError;
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
            .read(uow, command.currency_registrar_id)
            .await?;
        registrar.change_description(command.description.clone())?;
        self.repository
            .save(uow, request_context, &mut registrar)
            .await?;
        Ok(CurrencyRegistrarDescriptionChangeOutput::Changed)
    }
}
