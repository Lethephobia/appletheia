use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    CurrentDateTime, Organization, OrganizationId, OrganizationInvitation,
    OrganizationInvitationIssuance, OrganizationInvitationIssueRejectionReason,
    OrganizationInvitationIssuer, OrganizationMembership, OrganizationMembershipState, User,
    UserId,
};

use crate::authorization::{OrganizationInviterRelation, UserOwnerRelation};

use super::{
    OrganizationInvitationIssueCommand, OrganizationInvitationIssueCommandHandlerError,
    OrganizationInvitationIssueOutput,
};

/// Handles `OrganizationInvitationIssueCommand`.
pub struct OrganizationInvitationIssueCommandHandler<ORG, IR, MR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    organization_repository: ORG,
    organization_invitation_repository: IR,
    organization_membership_repository: MR,
}

impl<ORG, IR, MR> OrganizationInvitationIssueCommandHandler<ORG, IR, MR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    pub fn new(
        organization_repository: ORG,
        organization_invitation_repository: IR,
        organization_membership_repository: MR,
    ) -> Self {
        Self {
            organization_repository,
            organization_invitation_repository,
            organization_membership_repository,
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

impl<ORG, IR, MR> CommandHandler for OrganizationInvitationIssueCommandHandler<ORG, IR, MR>
where
    ORG: Repository<Organization>,
    IR: Repository<OrganizationInvitation, Uow = ORG::Uow>,
    MR: Repository<OrganizationMembership, Uow = ORG::Uow>,
{
    type Command = OrganizationInvitationIssueCommand;
    type Output = OrganizationInvitationIssueOutput;
    type ReplayOutput = OrganizationInvitationIssueOutput;
    type Error = OrganizationInvitationIssueCommandHandlerError;
    type Uow = ORG::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        let principal_requirement = match command.issuer {
            OrganizationInvitationIssuer::System => PrincipalRequirement::System,
            OrganizationInvitationIssuer::User(user_id) => {
                PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::All(
                    vec![
                        RelationshipRequirement::check::<User>(user_id, UserOwnerRelation::REF),
                        RelationshipRequirement::check::<Organization>(
                            command.organization_id,
                            OrganizationInviterRelation::REF,
                        ),
                    ],
                ))
            }
        };

        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            principal_requirement,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(OrganizationInvitationIssueCommandHandlerError::OrganizationNotFound);
        };

        let invitee_unique_value =
            Self::organization_user_unique_value(command.organization_id, command.invitee_id)?;
        let mut organization_invitation = OrganizationInvitation::default();
        let issuance = OrganizationInvitationIssuance {
            organization_id: command.organization_id,
            invitee_id: command.invitee_id,
            roles: command.roles.clone(),
            issuer: command.issuer,
            expires_at: command.expires_at,
        };

        if organization.is_removed()? {
            let reason = OrganizationInvitationIssueRejectionReason::OrganizationRemoved;
            let organization_invitation_id =
                organization_invitation.reject_issue(issuance, reason)?;

            self.organization_invitation_repository
                .save(uow, request_context, &mut organization_invitation)
                .await?;

            return Ok(CommandHandled::same(
                OrganizationInvitationIssueOutput::Rejected {
                    organization_invitation_id,
                    reason,
                },
            ));
        }

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
            let reason = OrganizationInvitationIssueRejectionReason::InviteeAlreadyMember;
            let organization_invitation_id =
                organization_invitation.reject_issue(issuance, reason)?;

            self.organization_invitation_repository
                .save(uow, request_context, &mut organization_invitation)
                .await?;

            return Ok(CommandHandled::same(
                OrganizationInvitationIssueOutput::Rejected {
                    organization_invitation_id,
                    reason,
                },
            ));
        }

        let result = organization_invitation.issue(issuance, CurrentDateTime::new())?;

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        let output = match result {
            banking_iam_domain::OrganizationInvitationIssueResult::Issued {
                organization_invitation_id,
            } => OrganizationInvitationIssueOutput::Issued {
                organization_invitation_id,
            },
            banking_iam_domain::OrganizationInvitationIssueResult::Rejected {
                organization_invitation_id,
                reason,
            } => OrganizationInvitationIssueOutput::Rejected {
                organization_invitation_id,
                reason,
            },
        };

        Ok(CommandHandled::same(output))
    }
}
