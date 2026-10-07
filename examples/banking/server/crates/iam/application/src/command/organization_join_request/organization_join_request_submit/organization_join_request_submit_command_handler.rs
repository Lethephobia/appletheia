use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::OrganizationJoinRequestError;
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationJoinRequest, OrganizationJoinRequestState,
    OrganizationMembership, OrganizationMembershipState, User, UserId,
};

use super::{
    OrganizationJoinRequestSubmitCommand, OrganizationJoinRequestSubmitCommandHandlerError,
    OrganizationJoinRequestSubmitOutput,
};
use crate::authorization::UserOwnerRelation;

pub struct OrganizationJoinRequestSubmitCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationJoinRequestSubmitCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn organization_requester_unique_value(
        organization_id: OrganizationId,
        requester_id: UserId,
    ) -> Result<UniqueValue, OrganizationJoinRequestSubmitCommandHandlerError> {
        let organization_value = organization_id.value().to_string();
        let requester_value = requester_id.value().to_string();
        let organization_part = UniqueValuePart::try_from(organization_value.as_str())?;
        let requester_part = UniqueValuePart::try_from(requester_value.as_str())?;
        Ok(UniqueValue::new(vec![organization_part, requester_part])?)
    }
}

impl<R> CommandHandler for OrganizationJoinRequestSubmitCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationJoinRequestSubmitCommand;
    type Output = OrganizationJoinRequestSubmitOutput;
    type Error = OrganizationJoinRequestSubmitCommandHandlerError;
    type Uow = R::Uow;

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
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_join_request = OrganizationJoinRequest::new();
        let organization_join_request_id = organization_join_request.aggregate_id();

        let organization = self
            .repository
            .read::<Organization>(uow, command.organization_id)
            .await?;
        if organization.is_removed()? {
            return Err(OrganizationJoinRequestError::OrganizationRemoved.into());
        }

        let membership_unique_value = Self::organization_requester_unique_value(
            command.organization_id,
            command.requester_id,
        )?;
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
            return Err(OrganizationJoinRequestError::RequesterAlreadyMember.into());
        }

        let unique_value = Self::organization_requester_unique_value(
            command.organization_id,
            command.requester_id,
        )?;
        if self
            .repository
            .find_by_unique_value::<OrganizationJoinRequest>(
                uow,
                OrganizationJoinRequestState::ORGANIZATION_REQUESTER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(OrganizationJoinRequestError::AlreadySubmitted.into());
        }

        organization_join_request.submit(command.organization_id, command.requester_id)?;

        self.repository
            .save::<OrganizationJoinRequest>(uow, request_context, &mut organization_join_request)
            .await?;

        Ok(OrganizationJoinRequestSubmitOutput {
            organization_join_request_id,
        })
    }
}
