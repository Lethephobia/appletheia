use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestApproveResult,
};

use crate::authorization::CurrencyRegistrarJoinRequestApproverRelation;

use super::{
    CurrencyRegistrarJoinRequestApproveCommand,
    CurrencyRegistrarJoinRequestApproveCommandHandlerError,
    CurrencyRegistrarJoinRequestApproveOutput,
};

/// Handles `CurrencyRegistrarJoinRequestApproveCommand`.
pub struct CurrencyRegistrarJoinRequestApproveCommandHandler<JR>
where
    JR: Repository<CurrencyRegistrarJoinRequest>,
{
    currency_registrar_join_request_repository: JR,
}

impl<JR> CurrencyRegistrarJoinRequestApproveCommandHandler<JR>
where
    JR: Repository<CurrencyRegistrarJoinRequest>,
{
    pub fn new(currency_registrar_join_request_repository: JR) -> Self {
        Self {
            currency_registrar_join_request_repository,
        }
    }
}

impl<JR> CommandHandler for CurrencyRegistrarJoinRequestApproveCommandHandler<JR>
where
    JR: Repository<CurrencyRegistrarJoinRequest>,
{
    type Command = CurrencyRegistrarJoinRequestApproveCommand;
    type Output = CurrencyRegistrarJoinRequestApproveOutput;
    type Error = CurrencyRegistrarJoinRequestApproveCommandHandlerError;
    type Uow = JR::Uow;

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
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut currency_registrar_join_request = self
            .currency_registrar_join_request_repository
            .read(uow, command.currency_registrar_join_request_id)
            .await?;

        let result = currency_registrar_join_request.approve()?;

        self.currency_registrar_join_request_repository
            .save(uow, _request_context, &mut currency_registrar_join_request)
            .await?;

        let output = match result {
            CurrencyRegistrarJoinRequestApproveResult::Approved => {
                CurrencyRegistrarJoinRequestApproveOutput::Approved
            }
            CurrencyRegistrarJoinRequestApproveResult::Rejected { reason } => {
                CurrencyRegistrarJoinRequestApproveOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
