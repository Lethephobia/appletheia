use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue};
use banking_iam_domain::{
    Organization, OrganizationInvitation, OrganizationInvitationIssuance,
    OrganizationInvitationIssueRejectionReason, OrganizationInvitationIssueResult,
    OrganizationInvitationIssuer, OrganizationInvitationState, OrganizationMembership,
    OrganizationMembershipState, User,
};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

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
        command: &OrganizationInvitationIssueCommand,
    ) -> Result<UniqueValue, OrganizationInvitationIssueCommandHandlerError> {
        let organization_id = command.organization_id.value().to_string();
        let invitee_id = command.invitee_id.value().to_string();
        Ok(UniqueValue::from_strings([
            organization_id.as_str(),
            invitee_id.as_str(),
        ])?)
    }

    fn organization_invitee_unique_value(
        command: &OrganizationInvitationIssueCommand,
    ) -> Result<UniqueValue, OrganizationInvitationIssueCommandHandlerError> {
        let organization_id = command.organization_id.value().to_string();
        let invitee_id = command.invitee_id.value().to_string();
        Ok(UniqueValue::from_strings([
            organization_id.as_str(),
            invitee_id.as_str(),
        ])?)
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
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_invitation = OrganizationInvitation::new();
        let organization_invitation_id = organization_invitation.aggregate_id();
        let issuance = OrganizationInvitationIssuance {
            organization_id: command.organization_id,
            invitee_id: command.invitee_id,
            roles: command.roles.clone(),
            issuer: command.issuer,
            expires_at: command.expires_at,
        };

        let organization = self
            .organization_repository
            .read(uow, command.organization_id)
            .await?;
        if organization.is_removed()? {
            let reason = OrganizationInvitationIssueRejectionReason::OrganizationRemoved;
            organization_invitation.reject_issue(issuance, reason)?;

            self.organization_invitation_repository
                .save(uow, request_context, &mut organization_invitation)
                .await?;

            return Ok(OrganizationInvitationIssueOutput::Rejected {
                organization_invitation_id,
                reason,
            });
        }

        let membership_unique_value = Self::organization_user_unique_value(command)?;
        if self
            .organization_membership_repository
            .find_by_unique_value(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &membership_unique_value,
            )
            .await?
            .is_some()
        {
            let reason = OrganizationInvitationIssueRejectionReason::InviteeAlreadyMember;
            organization_invitation.reject_issue(issuance, reason)?;

            self.organization_invitation_repository
                .save(uow, request_context, &mut organization_invitation)
                .await?;

            return Ok(OrganizationInvitationIssueOutput::Rejected {
                organization_invitation_id,
                reason,
            });
        }

        let unique_value = Self::organization_invitee_unique_value(command)?;
        if self
            .organization_invitation_repository
            .find_by_unique_value(
                uow,
                OrganizationInvitationState::ORGANIZATION_INVITEE_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = OrganizationInvitationIssueRejectionReason::AlreadyIssued;
            organization_invitation.reject_issue(issuance, reason)?;

            self.organization_invitation_repository
                .save(uow, request_context, &mut organization_invitation)
                .await?;

            return Ok(OrganizationInvitationIssueOutput::Rejected {
                organization_invitation_id,
                reason,
            });
        }

        let result = organization_invitation.issue(issuance, CurrentDateTime::new())?;

        self.organization_invitation_repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        let output = match result {
            OrganizationInvitationIssueResult::Issued => {
                OrganizationInvitationIssueOutput::Issued {
                    organization_invitation_id,
                }
            }
            OrganizationInvitationIssueResult::Rejected { reason } => {
                OrganizationInvitationIssueOutput::Rejected {
                    organization_invitation_id,
                    reason,
                }
            }
        };

        Ok(output)
    }
}
