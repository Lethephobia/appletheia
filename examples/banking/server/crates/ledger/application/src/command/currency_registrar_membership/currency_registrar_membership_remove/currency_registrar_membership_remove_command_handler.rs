use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipRemoveResult,
};

use super::{
    CurrencyRegistrarMembershipRemoveCommand, CurrencyRegistrarMembershipRemoveCommandHandlerError,
    CurrencyRegistrarMembershipRemoveOutput,
};
use crate::authorization::CurrencyRegistrarMembershipRemoverRelation;

pub struct CurrencyRegistrarMembershipRemoveCommandHandler<R>
where
    R: Repository<CurrencyRegistrarMembership>,
{
    repository: R,
}

impl<R> CurrencyRegistrarMembershipRemoveCommandHandler<R>
where
    R: Repository<CurrencyRegistrarMembership>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarMembershipRemoveCommandHandler<R>
where
    R: Repository<CurrencyRegistrarMembership>,
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
            .read(uow, command.currency_registrar_membership_id)
            .await?;
        let result = membership.remove()?;
        self.repository
            .save(uow, request_context, &mut membership)
            .await?;

        let output = match result {
            CurrencyRegistrarMembershipRemoveResult::Removed => {
                CurrencyRegistrarMembershipRemoveOutput::Removed
            }
            CurrencyRegistrarMembershipRemoveResult::Rejected { reason } => {
                CurrencyRegistrarMembershipRemoveOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
