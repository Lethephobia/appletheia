use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::{
    CurrencyRegistrarJoinRequest, CurrencyRegistrarJoinRequestRejectResult,
};

use crate::authorization::CurrencyRegistrarJoinRequestRejecterRelation;

use super::{
    CurrencyRegistrarJoinRequestRejectCommand,
    CurrencyRegistrarJoinRequestRejectCommandHandlerError,
    CurrencyRegistrarJoinRequestRejectOutput,
};

/// Handles `CurrencyRegistrarJoinRequestRejectCommand`.
pub struct CurrencyRegistrarJoinRequestRejectCommandHandler<JR>
where
    JR: Repository<CurrencyRegistrarJoinRequest>,
{
    currency_registrar_join_request_repository: JR,
}

impl<JR> CurrencyRegistrarJoinRequestRejectCommandHandler<JR>
where
    JR: Repository<CurrencyRegistrarJoinRequest>,
{
    pub fn new(currency_registrar_join_request_repository: JR) -> Self {
        Self {
            currency_registrar_join_request_repository,
        }
    }
}

impl<JR> CommandHandler for CurrencyRegistrarJoinRequestRejectCommandHandler<JR>
where
    JR: Repository<CurrencyRegistrarJoinRequest>,
{
    type Command = CurrencyRegistrarJoinRequestRejectCommand;
    type Output = CurrencyRegistrarJoinRequestRejectOutput;
    type Error = CurrencyRegistrarJoinRequestRejectCommandHandlerError;
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
                CurrencyRegistrarJoinRequestRejecterRelation::REF,
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

        let result = currency_registrar_join_request.reject()?;

        self.currency_registrar_join_request_repository
            .save(uow, _request_context, &mut currency_registrar_join_request)
            .await?;

        let output = match result {
            CurrencyRegistrarJoinRequestRejectResult::Rejected => {
                CurrencyRegistrarJoinRequestRejectOutput::Rejected
            }
            CurrencyRegistrarJoinRequestRejectResult::RejectionRejected { reason } => {
                CurrencyRegistrarJoinRequestRejectOutput::RejectionRejected { reason }
            }
        };

        Ok(output)
    }
}
