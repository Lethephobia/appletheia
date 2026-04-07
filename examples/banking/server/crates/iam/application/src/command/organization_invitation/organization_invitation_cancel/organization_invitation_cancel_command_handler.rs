use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationRefOwned,
    RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{Organization, OrganizationInvitation};

use crate::authorization::OrganizationInvitationCancelerRelation;
use crate::projection::{
    OrganizationInvitationOrganizationRelationshipProjectorSpec,
    OrganizationOwnerRelationshipProjectorSpec,
};

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
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<OrganizationInvitation>(
                        command.organization_invitation_id,
                    ),
                    relation: RelationRefOwned::from(OrganizationInvitationCancelerRelation::REF),
                },
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationInvitationOrganizationRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
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

        if organization.is_removed()? {
            return Err(OrganizationInvitationCancelCommandHandlerError::OrganizationRemoved);
        }

        organization_invitation.cancel()?;

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        Ok(CommandHandled::same(OrganizationInvitationCancelOutput))
    }
}
