use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::{CurrencyRegistrarInvitation, CurrencyRegistrarInvitationAcceptResult};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::authorization::CurrencyRegistrarInvitationInviteeRelation;

use super::{
    CurrencyRegistrarInvitationAcceptCommand, CurrencyRegistrarInvitationAcceptCommandHandlerError,
    CurrencyRegistrarInvitationAcceptOutput,
};

/// Handles `CurrencyRegistrarInvitationAcceptCommand`.
pub struct CurrencyRegistrarInvitationAcceptCommandHandler<IR>
where
    IR: Repository<CurrencyRegistrarInvitation>,
{
    currency_registrar_invitation_repository: IR,
}

impl<IR> CurrencyRegistrarInvitationAcceptCommandHandler<IR>
where
    IR: Repository<CurrencyRegistrarInvitation>,
{
    pub fn new(currency_registrar_invitation_repository: IR) -> Self {
        Self {
            currency_registrar_invitation_repository,
        }
    }
}

impl<IR> CommandHandler for CurrencyRegistrarInvitationAcceptCommandHandler<IR>
where
    IR: Repository<CurrencyRegistrarInvitation>,
{
    type Command = CurrencyRegistrarInvitationAcceptCommand;
    type Output = CurrencyRegistrarInvitationAcceptOutput;
    type Error = CurrencyRegistrarInvitationAcceptCommandHandlerError;
    type Uow = IR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                CurrencyRegistrarInvitation,
            >(
                command.currency_registrar_invitation_id,
                CurrencyRegistrarInvitationInviteeRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut currency_registrar_invitation = self
            .currency_registrar_invitation_repository
            .read(uow, command.currency_registrar_invitation_id)
            .await?;

        let result = currency_registrar_invitation.accept(CurrentDateTime::new())?;

        self.currency_registrar_invitation_repository
            .save(uow, _request_context, &mut currency_registrar_invitation)
            .await?;

        let output = match result {
            CurrencyRegistrarInvitationAcceptResult::Accepted => {
                CurrencyRegistrarInvitationAcceptOutput::Accepted
            }
            CurrencyRegistrarInvitationAcceptResult::Rejected { reason } => {
                CurrencyRegistrarInvitationAcceptOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
