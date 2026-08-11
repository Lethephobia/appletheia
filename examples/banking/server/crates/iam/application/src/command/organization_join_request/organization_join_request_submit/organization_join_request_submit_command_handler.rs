use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_iam_domain::{
    Organization, OrganizationId, OrganizationJoinRequest, OrganizationJoinRequestState,
    OrganizationJoinRequestSubmission, OrganizationJoinRequestSubmitRejectionReason,
    OrganizationJoinRequestSubmitResult, User, UserId,
};

use super::{
    OrganizationJoinRequestSubmitCommand, OrganizationJoinRequestSubmitCommandHandlerError,
    OrganizationJoinRequestSubmitOutput,
};
use crate::authorization::UserOwnerRelation;

/// Handles `OrganizationJoinRequestSubmitCommand`.
pub struct OrganizationJoinRequestSubmitCommandHandler<OR, JR, UR>
where
    OR: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = OR::Uow>,
    UR: Repository<User, Uow = OR::Uow>,
{
    organization_repository: OR,
    organization_join_request_repository: JR,
    user_repository: UR,
}

impl<OR, JR, UR> OrganizationJoinRequestSubmitCommandHandler<OR, JR, UR>
where
    OR: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = OR::Uow>,
    UR: Repository<User, Uow = OR::Uow>,
{
    pub fn new(
        organization_repository: OR,
        organization_join_request_repository: JR,
        user_repository: UR,
    ) -> Self {
        Self {
            organization_repository,
            organization_join_request_repository,
            user_repository,
        }
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

impl<OR, JR, UR> CommandHandler for OrganizationJoinRequestSubmitCommandHandler<OR, JR, UR>
where
    OR: Repository<Organization>,
    JR: Repository<OrganizationJoinRequest, Uow = OR::Uow>,
    UR: Repository<User, Uow = OR::Uow>,
{
    type Command = OrganizationJoinRequestSubmitCommand;
    type Output = OrganizationJoinRequestSubmitOutput;
    type Error = OrganizationJoinRequestSubmitCommandHandlerError;
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
    ) -> Result<Self::Output, Self::Error> {
        let mut organization_join_request = OrganizationJoinRequest::new();
        let organization_join_request_id = organization_join_request.aggregate_id();
        let submission = OrganizationJoinRequestSubmission {
            organization_id: command.organization_id,
            requester_id: command.requester_id,
        };

        let organization = self
            .organization_repository
            .read(uow, command.organization_id)
            .await?;
        if organization.is_removed()? {
            let reason = OrganizationJoinRequestSubmitRejectionReason::OrganizationRemoved;
            organization_join_request.reject_submit(submission, reason)?;

            self.organization_join_request_repository
                .save(uow, request_context, &mut organization_join_request)
                .await?;

            return Ok(OrganizationJoinRequestSubmitOutput::Rejected {
                organization_join_request_id,
                reason,
            });
        }

        let requester = self.user_repository.read(uow, command.requester_id).await?;
        if requester.is_organization_member(command.organization_id)? {
            let reason = OrganizationJoinRequestSubmitRejectionReason::RequesterAlreadyMember;
            organization_join_request.reject_submit(submission, reason)?;

            self.organization_join_request_repository
                .save(uow, request_context, &mut organization_join_request)
                .await?;

            return Ok(OrganizationJoinRequestSubmitOutput::Rejected {
                organization_join_request_id,
                reason,
            });
        }

        let unique_value = Self::organization_requester_unique_value(
            command.organization_id,
            command.requester_id,
        )?;
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
            let reason = OrganizationJoinRequestSubmitRejectionReason::AlreadySubmitted;
            organization_join_request.reject_submit(submission, reason)?;

            self.organization_join_request_repository
                .save(uow, request_context, &mut organization_join_request)
                .await?;

            return Ok(OrganizationJoinRequestSubmitOutput::Rejected {
                organization_join_request_id,
                reason,
            });
        }

        let result = organization_join_request.submit(submission)?;

        self.organization_join_request_repository
            .save(uow, request_context, &mut organization_join_request)
            .await?;

        let output = match result {
            OrganizationJoinRequestSubmitResult::Submitted => {
                OrganizationJoinRequestSubmitOutput::Submitted {
                    organization_join_request_id,
                }
            }
            OrganizationJoinRequestSubmitResult::Rejected { reason } => {
                OrganizationJoinRequestSubmitOutput::Rejected {
                    organization_join_request_id,
                    reason,
                }
            }
        };

        Ok(output)
    }
}
