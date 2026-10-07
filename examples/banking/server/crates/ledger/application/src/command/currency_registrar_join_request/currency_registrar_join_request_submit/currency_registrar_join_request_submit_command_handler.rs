use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use appletheia::domain::{AggregateId, UniqueValue, UniqueValuePart};
use banking_ledger_domain::currency_registrar_join_request::CurrencyRegistrarJoinRequestError;
use banking_ledger_domain::{
    CurrencyRegistrar, CurrencyRegistrarId, CurrencyRegistrarJoinRequest,
    CurrencyRegistrarJoinRequestState, CurrencyRegistrarMembership,
    CurrencyRegistrarMembershipState, User, UserId,
};

use super::{
    CurrencyRegistrarJoinRequestSubmitCommand,
    CurrencyRegistrarJoinRequestSubmitCommandHandlerError,
    CurrencyRegistrarJoinRequestSubmitOutput,
};
use crate::authorization::UserOwnerRelation;

pub struct CurrencyRegistrarJoinRequestSubmitCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarJoinRequestSubmitCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
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

impl<R> CommandHandler for CurrencyRegistrarJoinRequestSubmitCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarJoinRequestSubmitCommand;
    type Output = CurrencyRegistrarJoinRequestSubmitOutput;
    type Error = CurrencyRegistrarJoinRequestSubmitCommandHandlerError;
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
        let mut currency_registrar_join_request = CurrencyRegistrarJoinRequest::new();
        let currency_registrar_join_request_id = currency_registrar_join_request.aggregate_id();

        self.repository
            .read::<CurrencyRegistrar>(uow, command.currency_registrar_id)
            .await?;

        let membership_unique_value = Self::registrar_requester_unique_value(
            command.currency_registrar_id,
            command.requester_id,
        )?;
        if self
            .repository
            .find_by_unique_value::<CurrencyRegistrarMembership>(
                uow,
                CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY,
                &membership_unique_value,
            )
            .await?
            .is_some()
        {
            return Err(CurrencyRegistrarJoinRequestError::RequesterAlreadyMember.into());
        }

        let unique_value = Self::registrar_requester_unique_value(
            command.currency_registrar_id,
            command.requester_id,
        )?;
        if self
            .repository
            .find_by_unique_value::<CurrencyRegistrarJoinRequest>(
                uow,
                CurrencyRegistrarJoinRequestState::REGISTRAR_REQUESTER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(CurrencyRegistrarJoinRequestError::AlreadySubmitted.into());
        }

        currency_registrar_join_request
            .submit(command.currency_registrar_id, command.requester_id)?;

        self.repository
            .save::<CurrencyRegistrarJoinRequest>(
                uow,
                request_context,
                &mut currency_registrar_join_request,
            )
            .await?;

        Ok(CurrencyRegistrarJoinRequestSubmitOutput {
            currency_registrar_join_request_id,
        })
    }
}
