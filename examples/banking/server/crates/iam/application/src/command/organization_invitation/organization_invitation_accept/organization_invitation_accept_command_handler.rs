use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationInvitation;

use crate::authorization::OrganizationInvitationInviteeRelation;
use crate::projection::OrganizationInvitationRelationshipProjectorSpec;

use super::{
    OrganizationInvitationAcceptCommand, OrganizationInvitationAcceptCommandHandlerError,
    OrganizationInvitationAcceptOutput,
};

/// Handles `OrganizationInvitationAcceptCommand`.
pub struct OrganizationInvitationAcceptCommandHandler<OR>
where
    OR: Repository<OrganizationInvitation>,
{
    organization_invitation_repository: OR,
}

impl<OR> OrganizationInvitationAcceptCommandHandler<OR>
where
    OR: Repository<OrganizationInvitation>,
{
    pub fn new(organization_invitation_repository: OR) -> Self {
        Self {
            organization_invitation_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationInvitationAcceptCommandHandler<OR>
where
    OR: Repository<OrganizationInvitation>,
{
    type Command = OrganizationInvitationAcceptCommand;
    type Output = OrganizationInvitationAcceptOutput;
    type ReplayOutput = OrganizationInvitationAcceptOutput;
    type Error = OrganizationInvitationAcceptCommandHandlerError;
    type Uow = OR::Uow;

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
                    OrganizationInvitationRelationshipProjectorSpec::DESCRIPTOR,
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

        organization_invitation.accept()?;

        self.organization_invitation_repository
            .save(uow, _request_context, &mut organization_invitation)
            .await?;

        Ok(CommandHandled::same(OrganizationInvitationAcceptOutput))
    }
}
