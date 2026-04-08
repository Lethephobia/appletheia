use appletheia::application::authorization::AuthorizationPlan;
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::{Principal, RequestContext};
use appletheia::domain::{Aggregate, AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationJoinRequest, OrganizationJoinRequestState,
    OrganizationMembership, OrganizationMembershipState, User, UserId,
};

use super::{
    OrganizationJoinRequestCreateCommand, OrganizationJoinRequestCreateCommandHandlerError,
    OrganizationJoinRequestCreateOutput,
};

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

    fn requester_id(
        request_context: &RequestContext,
    ) -> Result<UserId, OrganizationJoinRequestCreateCommandHandlerError> {
        let Principal::Authenticated { subject } = &request_context.principal else {
            return Err(
                OrganizationJoinRequestCreateCommandHandlerError::JoinRequesterRequiresPrincipal,
            );
        };

        if subject.aggregate_type.value() != User::TYPE.value() {
            return Err(
                OrganizationJoinRequestCreateCommandHandlerError::JoinRequesterRequiresUserPrincipal,
            );
        }

        UserId::try_from_uuid(subject.aggregate_id.value())
            .map_err(OrganizationJoinRequestCreateCommandHandlerError::InvalidJoinRequesterUserId)
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
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            appletheia::application::authorization::PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let requester_id = Self::requester_id(request_context)?;

        let Some(organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(OrganizationJoinRequestCreateCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationJoinRequestCreateCommandHandlerError::OrganizationRemoved);
        }

        let unique_value =
            Self::organization_requester_unique_value(command.organization_id, requester_id)?;

        if self
            .organization_membership_repository
            .find_by_unique_value(
                uow,
                OrganizationMembershipState::ORGANIZATION_USER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationJoinRequestCreateCommandHandlerError::RequesterAlreadyMember);
        }

        if self
            .organization_join_request_repository
            .find_by_unique_value(
                uow,
                OrganizationJoinRequestState::ORGANIZATION_REQUESTER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(
                OrganizationJoinRequestCreateCommandHandlerError::JoinRequestAlreadyRequested,
            );
        }

        let mut organization_join_request = OrganizationJoinRequest::default();
        organization_join_request.request(command.organization_id, requester_id)?;

        self.organization_join_request_repository
            .save(uow, request_context, &mut organization_join_request)
            .await?;

        let organization_join_request_id = organization_join_request.aggregate_id().ok_or(
            OrganizationJoinRequestCreateCommandHandlerError::MissingOrganizationJoinRequestId,
        )?;

        Ok(CommandHandled::same(
            OrganizationJoinRequestCreateOutput::new(organization_join_request_id),
        ))
    }
}
