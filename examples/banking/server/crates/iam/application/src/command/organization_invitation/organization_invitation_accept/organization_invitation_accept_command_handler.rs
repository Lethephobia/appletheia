use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{
    Organization, OrganizationInvitation, OrganizationInvitationAcceptRejectionReason,
    OrganizationInvitationAcceptResult,
};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::authorization::OrganizationInvitationInviteeRelation;

use super::{
    OrganizationInvitationAcceptCommand, OrganizationInvitationAcceptCommandHandlerError,
    OrganizationInvitationAcceptOutput,
};

/// Handles `OrganizationInvitationAcceptCommand`.
pub struct OrganizationInvitationAcceptCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_invitation_repository: IR,
}

impl<ORG, IR> OrganizationInvitationAcceptCommandHandler<ORG, IR>
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

impl<ORG, IR> CommandHandler for OrganizationInvitationAcceptCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    type Command = OrganizationInvitationAcceptCommand;
    type Output = OrganizationInvitationAcceptOutput;
    type Error = OrganizationInvitationAcceptCommandHandlerError;
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
                OrganizationInvitationInviteeRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_invitation = self
            .organization_invitation_repository
            .read(uow, command.organization_invitation_id)
            .await?;

        let organization = self
            .organization_repository
            .read(uow, *organization_invitation.organization_id()?)
            .await?;

        if organization.is_removed()? {
            let reason = OrganizationInvitationAcceptRejectionReason::OrganizationRemoved;
            organization_invitation.reject_accept(reason)?;

            self.organization_invitation_repository
                .save(uow, _request_context, &mut organization_invitation)
                .await?;

            return Ok(OrganizationInvitationAcceptOutput::Rejected { reason });
        }

        let result = organization_invitation.accept(CurrentDateTime::new())?;

        self.organization_invitation_repository
            .save(uow, _request_context, &mut organization_invitation)
            .await?;

        let output = match result {
            OrganizationInvitationAcceptResult::Accepted => {
                OrganizationInvitationAcceptOutput::Accepted
            }
            OrganizationInvitationAcceptResult::Rejected { reason } => {
                OrganizationInvitationAcceptOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
