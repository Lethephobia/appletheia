use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationInvitation};

use crate::authorization::OrganizationInvitationInviteeRelation;
use crate::projection::OrganizationInvitationInviteeRelationshipProjectorSpec;

use super::{
    OrganizationInvitationDeclineCommand, OrganizationInvitationDeclineCommandHandlerError,
    OrganizationInvitationDeclineOutput,
};

/// Handles `OrganizationInvitationDeclineCommand`.
pub struct OrganizationInvitationDeclineCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_invitation_repository: IR,
}

impl<ORG, IR> OrganizationInvitationDeclineCommandHandler<ORG, IR>
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

impl<ORG, IR> CommandHandler for OrganizationInvitationDeclineCommandHandler<ORG, IR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
{
    type Command = OrganizationInvitationDeclineCommand;
    type Output = OrganizationInvitationDeclineOutput;
    type ReplayOutput = OrganizationInvitationDeclineOutput;
    type Error = OrganizationInvitationDeclineCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<OrganizationInvitation>(
                        command.organization_invitation_id,
                    ),
                    relation: OrganizationInvitationInviteeRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationInvitationInviteeRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
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
                OrganizationInvitationDeclineCommandHandlerError::TargetOrganizationInvitationNotFound,
            );
        };

        let Some(organization) = self
            .organization_repository
            .find(uow, *organization_invitation.organization_id()?)
            .await?
        else {
            return Err(OrganizationInvitationDeclineCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationInvitationDeclineCommandHandlerError::OrganizationRemoved);
        }

        organization_invitation.decline()?;

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        Ok(CommandHandled::same(OrganizationInvitationDeclineOutput))
    }
}
