use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, AggregateId, UniqueValue};
use banking_iam_domain::UserId;
use banking_ledger_domain::currency_registrar::{CurrencyRegistrar, CurrencyRegistrarId};
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembershipError;
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipState,
};

use super::{
    CurrencyRegistrarMembershipCreateCommand, CurrencyRegistrarMembershipCreateCommandHandlerError,
    CurrencyRegistrarMembershipCreateOutput,
};

pub struct CurrencyRegistrarMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> CurrencyRegistrarMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn registrar_user_unique_value(
        currency_registrar_id: CurrencyRegistrarId,
        user_id: UserId,
    ) -> Result<UniqueValue, CurrencyRegistrarMembershipCreateCommandHandlerError> {
        let currency_registrar_id = currency_registrar_id.value().to_string();
        let user_id = user_id.value().to_string();
        Ok(UniqueValue::from_strings([
            currency_registrar_id.as_str(),
            user_id.as_str(),
        ])?)
    }
}

impl<R> CommandHandler for CurrencyRegistrarMembershipCreateCommandHandler<R>
where
    R: Repository,
{
    type Command = CurrencyRegistrarMembershipCreateCommand;
    type Output = CurrencyRegistrarMembershipCreateOutput;
    type Error = CurrencyRegistrarMembershipCreateCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        self.repository
            .read::<CurrencyRegistrar>(uow, command.currency_registrar_id)
            .await?;

        let unique_value =
            Self::registrar_user_unique_value(command.currency_registrar_id, command.user_id)?;
        let mut membership = CurrencyRegistrarMembership::new();
        let currency_registrar_membership_id = membership.aggregate_id();
        if self
            .repository
            .find_by_unique_value::<CurrencyRegistrarMembership>(
                uow,
                CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            return Err(CurrencyRegistrarMembershipError::AlreadyMember.into());
        }

        membership.create(command.currency_registrar_id, command.user_id)?;
        self.repository
            .save(uow, request_context, &mut membership)
            .await?;
        Ok(CurrencyRegistrarMembershipCreateOutput {
            currency_registrar_membership_id,
        })
    }
}
