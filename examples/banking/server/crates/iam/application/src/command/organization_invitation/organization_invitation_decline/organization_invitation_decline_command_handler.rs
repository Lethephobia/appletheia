use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::OrganizationInvitation;

use crate::authorization::OrganizationInvitationInviteeRelation;
use crate::projection::OrganizationInvitationInviteeRelationshipProjectorSpec;

use super::{
    OrganizationInvitationDeclineCommand, OrganizationInvitationDeclineCommandHandlerError,
    OrganizationInvitationDeclineOutput,
};

/// Handles `OrganizationInvitationDeclineCommand`.
pub struct OrganizationInvitationDeclineCommandHandler<OR>
where
    OR: Repository<OrganizationInvitation>,
{
    organization_invitation_repository: OR,
}

impl<OR> OrganizationInvitationDeclineCommandHandler<OR>
where
    OR: Repository<OrganizationInvitation>,
{
    pub fn new(organization_invitation_repository: OR) -> Self {
        Self {
            organization_invitation_repository,
        }
    }
}

impl<OR> CommandHandler for OrganizationInvitationDeclineCommandHandler<OR>
where
    OR: Repository<OrganizationInvitation>,
{
    type Command = OrganizationInvitationDeclineCommand;
    type Output = OrganizationInvitationDeclineOutput;
    type ReplayOutput = OrganizationInvitationDeclineOutput;
    type Error = OrganizationInvitationDeclineCommandHandlerError;
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

        organization_invitation.decline()?;

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        Ok(CommandHandled::same(OrganizationInvitationDeclineOutput))
    }
}
