use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue};
use banking_iam_domain::OrganizationInvitationError;
use banking_iam_domain::{
    Organization, OrganizationInvitation, OrganizationInvitationIssuer,
    OrganizationInvitationState, OrganizationMembership, OrganizationMembershipState, User,
};
use banking_shared_kernel_domain::timestamps::CurrentDateTime;

use crate::authorization::{OrganizationInviterRelation, UserOwnerRelation};

use super::{
    OrganizationInvitationIssueCommand, OrganizationInvitationIssueCommandHandlerError,
    OrganizationInvitationIssueOutput,
};

pub struct OrganizationInvitationIssueCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationInvitationIssueCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
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

impl<R> CommandHandler for OrganizationInvitationIssueCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationInvitationIssueCommand;
    type Output = OrganizationInvitationIssueOutput;
    type Error = OrganizationInvitationIssueCommandHandlerError;
    type Uow = R::Uow;

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

        let organization = self
            .repository
            .read::<Organization>(uow, command.organization_id)
            .await?;
        if organization.is_removed()? {
            return Err(OrganizationInvitationError::OrganizationRemoved.into());
        }

        let membership_unique_value = Self::organization_user_unique_value(command)?;
        if self
            .repository
            .find_by_unique_value::<OrganizationMembership>(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &membership_unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationInvitationError::InviteeAlreadyMember.into());
        }

        let unique_value = Self::organization_invitee_unique_value(command)?;
        if self
            .repository
            .find_by_unique_value::<OrganizationInvitation>(
                uow,
                OrganizationInvitationState::ORGANIZATION_INVITEE_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationInvitationError::AlreadyIssued.into());
        }

        organization_invitation.issue(
            command.organization_id,
            command.invitee_id,
            command.roles.clone(),
            command.issuer,
            command.expires_at,
            CurrentDateTime::new(),
        )?;

        self.repository
            .save(uow, request_context, &mut organization_invitation)
            .await?;

        Ok(OrganizationInvitationIssueOutput {
            organization_invitation_id,
        })
    }
}
