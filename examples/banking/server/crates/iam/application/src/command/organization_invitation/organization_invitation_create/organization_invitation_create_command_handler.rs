use appletheia::application::authorization::{
    AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::{Principal, RequestContext};
use appletheia::domain::{Aggregate, AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationInvitation, OrganizationInvitationIssuer,
    OrganizationMembership, OrganizationMembershipState, User, UserId,
};

use crate::authorization::OrganizationInviterRelation;
use crate::projection::OrganizationOwnerRelationshipProjectorSpec;

use super::{
    OrganizationInvitationIssueCommand, OrganizationInvitationIssueCommandHandlerError,
    OrganizationInvitationIssueOutput,
};

/// Handles `OrganizationInvitationIssueCommand`.
pub struct OrganizationInvitationIssueCommandHandler<IR, MR>
where
    IR: Repository<OrganizationInvitation, Uow = MR::Uow>,
    MR: Repository<OrganizationMembership>,
{
    organization_invitation_repository: IR,
    organization_membership_repository: MR,
}

impl<IR, MR> OrganizationInvitationIssueCommandHandler<IR, MR>
where
    IR: Repository<OrganizationInvitation, Uow = MR::Uow>,
    MR: Repository<OrganizationMembership>,
{
    pub fn new(
        organization_invitation_repository: IR,
        organization_membership_repository: MR,
    ) -> Self {
        Self {
            organization_invitation_repository,
            organization_membership_repository,
        }
    }

    fn issuer(
        request_context: &RequestContext,
    ) -> Result<OrganizationInvitationIssuer, OrganizationInvitationIssueCommandHandlerError> {
        match &request_context.principal {
            Principal::System => Ok(OrganizationInvitationIssuer::System),
            Principal::Authenticated { subject } => {
                if subject.aggregate_type.value() != User::TYPE.value() {
                    return Err(
                        OrganizationInvitationIssueCommandHandlerError::InvitationIssuerRequiresUserPrincipal,
                    );
                }

                Ok(OrganizationInvitationIssuer::User(
                    UserId::try_from_uuid(subject.aggregate_id.value()).map_err(
                        OrganizationInvitationIssueCommandHandlerError::InvalidInvitationIssuerUserId,
                    )?,
                ))
            }
            Principal::Anonymous | Principal::Unavailable => Err(
                OrganizationInvitationIssueCommandHandlerError::InvitationIssuerRequiresPrincipal,
            ),
        }
    }

    fn organization_user_unique_value(
        organization_id: OrganizationId,
        invitee_id: UserId,
    ) -> Result<UniqueValue, OrganizationInvitationIssueCommandHandlerError> {
        let organization_value = organization_id.value().to_string();
        let invitee_value = invitee_id.value().to_string();
        let organization_part = UniqueValuePart::try_from(organization_value.as_str())?;
        let invitee_part = UniqueValuePart::try_from(invitee_value.as_str())?;
        Ok(UniqueValue::new(vec![organization_part, invitee_part])?)
    }
}

impl<IR, MR> CommandHandler for OrganizationInvitationIssueCommandHandler<IR, MR>
where
    IR: Repository<OrganizationInvitation, Uow = MR::Uow>,
    MR: Repository<OrganizationMembership>,
{
    type Command = OrganizationInvitationIssueCommand;
    type Output = OrganizationInvitationIssueOutput;
    type ReplayOutput = OrganizationInvitationIssueOutput;
    type Error = OrganizationInvitationIssueCommandHandlerError;
    type Uow = IR::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::Check {
                    aggregate: AggregateRef::from_id::<Organization>(_command.organization_id),
                    relation: OrganizationInviterRelation::NAME,
                },
                projector_dependencies: ProjectorDependencies::Some(&[
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
        let invitee_unique_value =
            Self::organization_user_unique_value(command.organization_id, command.invitee_id)?;
        if self
            .organization_membership_repository
            .find_by_unique_value(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &invitee_unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationInvitationIssueCommandHandlerError::InviteeAlreadyMember);
        }

        let issuer = Self::issuer(request_context)?;

        let mut organization_invitation = OrganizationInvitation::default();
        organization_invitation.issue(
            command.organization_id,
            command.invitee_id,
            issuer,
            command.expires_at,
        )?;

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        let organization_invitation_id = organization_invitation.aggregate_id().ok_or(
            OrganizationInvitationIssueCommandHandlerError::MissingOrganizationInvitationId,
        )?;

        Ok(CommandHandled::same(
            OrganizationInvitationIssueOutput::new(organization_invitation_id),
        ))
    }
}
