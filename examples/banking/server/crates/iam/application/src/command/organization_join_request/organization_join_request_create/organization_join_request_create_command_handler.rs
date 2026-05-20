use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationJoinRequest,
    OrganizationJoinRequestRequestRejectionReason, OrganizationJoinRequestState,
    OrganizationMembership, OrganizationMembershipState, User, UserId,
};

use super::{
    OrganizationJoinRequestCreateCommand, OrganizationJoinRequestCreateCommandHandlerError,
    OrganizationJoinRequestCreateOutput,
};
use crate::authorization::UserOwnerRelation;

/// Handles `OrganizationJoinRequestCreateCommand`.
pub struct OrganizationJoinRequestCreateCommandHandler<OR, JR, MR>
where
    OR: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = OR::Uow>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
{
    organization_repository: OR,
    organization_join_request_repository: JR,
    organization_membership_repository: MR,
}

impl<OR, JR, MR> OrganizationJoinRequestCreateCommandHandler<OR, JR, MR>
where
    OR: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = OR::Uow>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
{
    pub fn new(
        organization_repository: OR,
        organization_join_request_repository: JR,
        organization_membership_repository: MR,
    ) -> Self {
        Self {
            organization_repository,
            organization_join_request_repository,
            organization_membership_repository,
        }
    }

    fn organization_requester_unique_value(
        organization_id: OrganizationId,
        requester_id: UserId,
    ) -> Result<UniqueValue, OrganizationJoinRequestCreateCommandHandlerError> {
        let organization_value = organization_id.value().to_string();
        let requester_value = requester_id.value().to_string();
        let organization_part = UniqueValuePart::try_from(organization_value.as_str())?;
        let requester_part = UniqueValuePart::try_from(requester_value.as_str())?;
        Ok(UniqueValue::new(vec![organization_part, requester_part])?)
    }
}

impl<OR, JR, MR> CommandHandler for OrganizationJoinRequestCreateCommandHandler<OR, JR, MR>
where
    OR: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = OR::Uow>,
    MR: Repository<OrganizationMembership, Uow = OR::Uow>,
{
    type Command = OrganizationJoinRequestCreateCommand;
    type Output = OrganizationJoinRequestCreateOutput;
    type ReplayOutput = OrganizationJoinRequestCreateOutput;
    type Error = OrganizationJoinRequestCreateCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                User,
            >(
                command.requester_id,
                UserOwnerRelation::REF,
            )),
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
            return Err(OrganizationJoinRequestCreateCommandHandlerError::OrganizationNotFound);
        };

        let unique_value = Self::organization_requester_unique_value(
            command.organization_id,
            command.requester_id,
        )?;
        let mut organization_join_request = OrganizationJoinRequest::default();
        let result = if organization.is_removed()? {
            let reason = OrganizationJoinRequestRequestRejectionReason::OrganizationRemoved;
            let organization_join_request_id = organization_join_request.reject_request(
                command.organization_id,
                command.requester_id,
                reason,
            )?;
            banking_iam_domain::OrganizationJoinRequestRequestResult::Rejected {
                organization_join_request_id,
                reason,
            }
        } else if self
            .organization_membership_repository
            .find_by_unique_value(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = OrganizationJoinRequestRequestRejectionReason::RequesterAlreadyMember;
            let organization_join_request_id = organization_join_request.reject_request(
                command.organization_id,
                command.requester_id,
                reason,
            )?;
            banking_iam_domain::OrganizationJoinRequestRequestResult::Rejected {
                organization_join_request_id,
                reason,
            }
        } else if self
            .organization_join_request_repository
            .find_by_unique_value(
                uow,
                OrganizationJoinRequestState::ORGANIZATION_REQUESTER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = OrganizationJoinRequestRequestRejectionReason::AlreadyRequested;
            let organization_join_request_id = organization_join_request.reject_request(
                command.organization_id,
                command.requester_id,
                reason,
            )?;
            banking_iam_domain::OrganizationJoinRequestRequestResult::Rejected {
                organization_join_request_id,
                reason,
            }
        } else {
            organization_join_request.request(command.organization_id, command.requester_id)?
        };

        self.organization_join_request_repository
            .save(uow, request_context, &mut organization_join_request)
            .await?;

        let output = match result {
            banking_iam_domain::OrganizationJoinRequestRequestResult::Requested {
                organization_join_request_id,
            } => OrganizationJoinRequestCreateOutput::Requested {
                organization_join_request_id,
            },
            banking_iam_domain::OrganizationJoinRequestRequestResult::Rejected {
                organization_join_request_id,
                reason,
            } => OrganizationJoinRequestCreateOutput::Rejected {
                organization_join_request_id,
                reason,
            },
        };

        Ok(CommandHandled::same(output))
    }
}
