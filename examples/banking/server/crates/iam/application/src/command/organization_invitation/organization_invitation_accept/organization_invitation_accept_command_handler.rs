use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationInvitation};

use crate::authorization::OrganizationInvitationInviteeRelation;
use crate::projection::OrganizationInvitationInviteeRelationshipProjectorSpec;

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
    type ReplayOutput = OrganizationInvitationAcceptOutput;
    type Error = OrganizationInvitationAcceptCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<OrganizationInvitation>(
                    command.organization_invitation_id,
                    OrganizationInvitationInviteeRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationInvitationInviteeRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(mut organization_invitation) = self
            .organization_invitation_repository
            .find(uow, command.organization_invitation_id)
            .await?
        else {
            return Err(
                OrganizationInvitationAcceptCommandHandlerError::TargetOrganizationInvitationNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_invitation.organization_id()?)
            .await?
        else {
            return Err(OrganizationInvitationAcceptCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationInvitationAcceptCommandHandlerError::OrganizationRemoved);
        }

        organization_invitation.accept()?;

        self.organization_invitation_repository
            .save(uow, _request_context, &mut organization_invitation)
            .await?;

        Ok(CommandHandled::same(OrganizationInvitationAcceptOutput))
    }
}
