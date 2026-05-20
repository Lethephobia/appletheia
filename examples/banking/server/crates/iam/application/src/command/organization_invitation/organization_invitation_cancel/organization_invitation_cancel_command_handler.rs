use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    CurrentDateTime, Organization, OrganizationInvitation,
    OrganizationInvitationCancelRejectionReason, OrganizationInvitationCancelResult,
};

use crate::authorization::OrganizationInvitationCancelerRelation;

use super::{
    OrganizationInvitationCancelCommand, OrganizationInvitationCancelCommandHandlerError,
    OrganizationInvitationCancelOutput,
};

/// Handles `OrganizationInvitationCancelCommand`.
pub struct OrganizationInvitationCancelCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_invitation_repository: IR,
}

impl<ORG, IR> OrganizationInvitationCancelCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    pub fn new(organization_repository: ORG, organization_invitation_repository: IR) -> Self {
        Self {
            organization_repository,
            organization_invitation_repository,
        }
    }
}

impl<ORG, IR> CommandHandler for OrganizationInvitationCancelCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    type Command = OrganizationInvitationCancelCommand;
    type Output = OrganizationInvitationCancelOutput;
    type ReplayOutput = OrganizationInvitationCancelOutput;
    type Error = OrganizationInvitationCancelCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                OrganizationInvitation,
            >(
                command.organization_invitation_id,
                OrganizationInvitationCancelerRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut organization_invitation) = self
            .organization_invitation_repository
            .find(uow, command.organization_invitation_id)
            .await?
        else {
            return Err(
                OrganizationInvitationCancelCommandHandlerError::TargetOrganizationInvitationNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_invitation.organization_id()?)
            .await?
        else {
            return Err(OrganizationInvitationCancelCommandHandlerError::OrganizationNotFound);
        };

        let result = if organization.is_removed()? {
            let reason = OrganizationInvitationCancelRejectionReason::OrganizationRemoved;
            organization_invitation.reject_cancel(reason)?;
            OrganizationInvitationCancelResult::Rejected { reason }
        } else {
            organization_invitation.cancel(CurrentDateTime::new())?
        };

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        let output = match result {
            OrganizationInvitationCancelResult::Canceled => {
                OrganizationInvitationCancelOutput::Canceled
            }
            OrganizationInvitationCancelResult::Rejected { reason } => {
                OrganizationInvitationCancelOutput::Rejected { reason }
            }
        };

        Ok(CommandHandled::same(output))
    }
}
