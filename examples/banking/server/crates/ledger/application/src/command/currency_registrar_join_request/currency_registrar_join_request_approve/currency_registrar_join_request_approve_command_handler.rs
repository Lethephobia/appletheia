use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::CurrencyRegistrarJoinRequest;

use crate::authorization::CurrencyRegistrarJoinRequestApproverRelation;

use super::{
    CurrencyRegistrarJoinRequestApproveCommand,
    CurrencyRegistrarJoinRequestApproveCommandHandlerError,
    CurrencyRegistrarJoinRequestApproveOutput,
};

pub struct CurrencyRegistrarJoinRequestApproveCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarJoinRequestApproveCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> CommandHandler for CurrencyRegistrarJoinRequestApproveCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarJoinRequestApproveCommand;
    type Output = CurrencyRegistrarJoinRequestApproveOutput;
    type Error = CurrencyRegistrarJoinRequestApproveCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                CurrencyRegistrarJoinRequest,
            >(
                command.currency_registrar_join_request_id,
                CurrencyRegistrarJoinRequestApproverRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut currency_registrar_join_request = self
            .repository
            .read::<CurrencyRegistrarJoinRequest>(uow, command.currency_registrar_join_request_id)
            .await?;

        currency_registrar_join_request.approve()?;

        self.repository
            .save(uow, request_context, &mut currency_registrar_join_request)
            .await?;

        Ok(CurrencyRegistrarJoinRequestApproveOutput {})
    }
}
