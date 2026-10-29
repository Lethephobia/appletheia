use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembership;

use super::{
    CurrencyRegistrarMembershipDeleteCommand, CurrencyRegistrarMembershipDeleteCommandHandlerError,
    CurrencyRegistrarMembershipDeleteOutput,
};
use crate::currency_registrar::authorization::CurrencyRegistrarMembershipManagerRelation;

pub struct CurrencyRegistrarMembershipDeleteCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarMembershipDeleteCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarMembershipDeleteCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarMembershipDeleteCommand;
    type Output = CurrencyRegistrarMembershipDeleteOutput;
    type Error = CurrencyRegistrarMembershipDeleteCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                CurrencyRegistrarMembership,
            >(
                command.currency_registrar_membership_id,
                CurrencyRegistrarMembershipManagerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut membership = self
            .repository
            .read::<CurrencyRegistrarMembership>(uow, command.currency_registrar_membership_id)
            .await?;
        membership.delete()?;
        self.repository
            .save(uow, request_context, &mut membership)
            .await?;

        Ok(CurrencyRegistrarMembershipDeleteOutput {})
    }
}
