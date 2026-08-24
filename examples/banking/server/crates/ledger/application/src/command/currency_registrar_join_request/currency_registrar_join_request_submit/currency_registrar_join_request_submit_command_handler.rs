use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarId, CurrencyRegistrarJoinRequest,
    CurrencyRegistrarJoinRequestState, CurrencyRegistrarJoinRequestSubmission,
    CurrencyRegistrarJoinRequestSubmitRejectionReason, CurrencyRegistrarJoinRequestSubmitResult,
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipState, User, UserId,
};

use super::{
    CurrencyRegistrarJoinRequestSubmitCommand,
    CurrencyRegistrarJoinRequestSubmitCommandHandlerError,
    CurrencyRegistrarJoinRequestSubmitOutput,
};
use crate::authorization::UserOwnerRelation;

/// Handles `CurrencyRegistrarJoinRequestSubmitCommand`.
pub struct CurrencyRegistrarJoinRequestSubmitCommandHandler<OR, JR, MR>
where
    OR: Repository<CurrencyRegistrar>,
    JR: Repository<CurrencyRegistrarJoinRequest, Uow = OR::Uow>,
    MR: Repository<CurrencyRegistrarMembership, Uow = OR::Uow>,
{
    currency_registrar_repository: OR,
    currency_registrar_join_request_repository: JR,
    membership_repository: MR,
}

impl<OR, JR, MR> CurrencyRegistrarJoinRequestSubmitCommandHandler<OR, JR, MR>
where
    OR: Repository<CurrencyRegistrar>,
    JR: Repository<CurrencyRegistrarJoinRequest, Uow = OR::Uow>,
    MR: Repository<CurrencyRegistrarMembership, Uow = OR::Uow>,
{
    pub fn new(
        currency_registrar_repository: OR,
        currency_registrar_join_request_repository: JR,
        membership_repository: MR,
    ) -> Self {
        Self {
            currency_registrar_repository,
            currency_registrar_join_request_repository,
            membership_repository,
        }
    }

    fn registrar_requester_unique_value(
        currency_registrar_id: CurrencyRegistrarId,
        requester_id: UserId,
    ) -> Result<UniqueValue, CurrencyRegistrarJoinRequestSubmitCommandHandlerError> {
        let registrar_value = currency_registrar_id.value().to_string();
        let requester_value = requester_id.value().to_string();
        let registrar_part = UniqueValuePart::try_from(registrar_value.as_str())?;
        let requester_part = UniqueValuePart::try_from(requester_value.as_str())?;
        Ok(UniqueValue::new(vec![registrar_part, requester_part])?)
    }
}

impl<OR, JR, MR> CommandHandler for CurrencyRegistrarJoinRequestSubmitCommandHandler<OR, JR, MR>
where
    OR: Repository<CurrencyRegistrar>,
    JR: Repository<CurrencyRegistrarJoinRequest, Uow = OR::Uow>,
    MR: Repository<CurrencyRegistrarMembership, Uow = OR::Uow>,
{
    type Command = CurrencyRegistrarJoinRequestSubmitCommand;
    type Output = CurrencyRegistrarJoinRequestSubmitOutput;
    type Error = CurrencyRegistrarJoinRequestSubmitCommandHandlerError;
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
        let mut currency_registrar_join_request = CurrencyRegistrarJoinRequest::new();
        let currency_registrar_join_request_id = currency_registrar_join_request.aggregate_id();
        let submission = CurrencyRegistrarJoinRequestSubmission {
            currency_registrar_id: command.currency_registrar_id,
            requester_id: command.requester_id,
        };

        self.currency_registrar_repository
            .read(uow, command.currency_registrar_id)
            .await?;

        let membership_unique_value = Self::registrar_requester_unique_value(
            command.currency_registrar_id,
            command.requester_id,
        )?;
        if self
            .membership_repository
            .find_by_unique_value(
                uow,
                CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY,
                &membership_unique_value,
            )
            .await?
            .is_some()
        {
            let reason = CurrencyRegistrarJoinRequestSubmitRejectionReason::RequesterAlreadyMember;
            currency_registrar_join_request.reject_submit(submission, reason)?;

            self.currency_registrar_join_request_repository
                .save(uow, request_context, &mut currency_registrar_join_request)
                .await?;

            return Ok(CurrencyRegistrarJoinRequestSubmitOutput::Rejected {
                currency_registrar_join_request_id,
                reason,
            });
        }

        let unique_value = Self::registrar_requester_unique_value(
            command.currency_registrar_id,
            command.requester_id,
        )?;
        if self
            .currency_registrar_join_request_repository
            .find_by_unique_value(
                uow,
                CurrencyRegistrarJoinRequestState::REGISTRAR_REQUESTER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = CurrencyRegistrarJoinRequestSubmitRejectionReason::AlreadySubmitted;
            currency_registrar_join_request.reject_submit(submission, reason)?;

            self.currency_registrar_join_request_repository
                .save(uow, request_context, &mut currency_registrar_join_request)
                .await?;

            return Ok(CurrencyRegistrarJoinRequestSubmitOutput::Rejected {
                currency_registrar_join_request_id,
                reason,
            });
        }

        let result = currency_registrar_join_request.submit(submission)?;

        self.currency_registrar_join_request_repository
            .save(uow, request_context, &mut currency_registrar_join_request)
            .await?;

        let output = match result {
            CurrencyRegistrarJoinRequestSubmitResult::Submitted => {
                CurrencyRegistrarJoinRequestSubmitOutput::Submitted {
                    currency_registrar_join_request_id,
                }
            }
            CurrencyRegistrarJoinRequestSubmitResult::Rejected { reason } => {
                CurrencyRegistrarJoinRequestSubmitOutput::Rejected {
                    currency_registrar_join_request_id,
                    reason,
                }
            }
        };

        Ok(output)
    }
}
