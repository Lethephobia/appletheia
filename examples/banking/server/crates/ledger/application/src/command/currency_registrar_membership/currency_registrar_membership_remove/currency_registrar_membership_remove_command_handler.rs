use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembership;

use super::{
    CurrencyRegistrarMembershipRemoveCommand, CurrencyRegistrarMembershipRemoveCommandHandlerError,
    CurrencyRegistrarMembershipRemoveOutput,
};
use crate::authorization::CurrencyRegistrarMembershipRemoverRelation;

pub struct CurrencyRegistrarMembershipRemoveCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarMembershipRemoveCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarMembershipRemoveCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarMembershipRemoveCommand;
    type Output = CurrencyRegistrarMembershipRemoveOutput;
    type Error = CurrencyRegistrarMembershipRemoveCommandHandlerError;
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
                CurrencyRegistrarMembershipRemoverRelation::REF,
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
        membership.remove()?;
        self.repository
            .save::<CurrencyRegistrarMembership>(uow, request_context, &mut membership)
            .await?;

        Ok(CurrencyRegistrarMembershipRemoveOutput {})
    }
}
