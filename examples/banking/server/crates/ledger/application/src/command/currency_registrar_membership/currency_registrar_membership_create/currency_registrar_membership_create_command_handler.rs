use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, AggregateId, UniqueValue};
use banking_iam_domain::UserId;
use banking_ledger_domain::currency_registrar::{CurrencyRegistrar, CurrencyRegistrarId};
use banking_ledger_domain::currency_registrar_membership::{
    CurrencyRegistrarMembership, CurrencyRegistrarMembershipCreateRejectionReason,
    CurrencyRegistrarMembershipCreateResult, CurrencyRegistrarMembershipState,
};

use super::{
    CurrencyRegistrarMembershipCreateCommand, CurrencyRegistrarMembershipCreateCommandHandlerError,
    CurrencyRegistrarMembershipCreateOutput,
};

pub struct CurrencyRegistrarMembershipCreateCommandHandler<RR, MR>
where
    RR: Repository<CurrencyRegistrar>,
    MR: Repository<CurrencyRegistrarMembership, Uow = RR::Uow>,
{
    currency_registrar_repository: RR,
    currency_registrar_membership_repository: MR,
}

impl<RR, MR> CurrencyRegistrarMembershipCreateCommandHandler<RR, MR>
where
    RR: Repository<CurrencyRegistrar>,
    MR: Repository<CurrencyRegistrarMembership, Uow = RR::Uow>,
{
    pub fn new(
        currency_registrar_repository: RR,
        currency_registrar_membership_repository: MR,
    ) -> Self {
        Self {
            currency_registrar_repository,
            currency_registrar_membership_repository,
        }
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

impl<RR, MR> CommandHandler for CurrencyRegistrarMembershipCreateCommandHandler<RR, MR>
where
    RR: Repository<CurrencyRegistrar>,
    MR: Repository<CurrencyRegistrarMembership, Uow = RR::Uow>,
{
    type Command = CurrencyRegistrarMembershipCreateCommand;
    type Output = CurrencyRegistrarMembershipCreateOutput;
    type Error = CurrencyRegistrarMembershipCreateCommandHandlerError;
    type Uow = RR::Uow;

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
        self.currency_registrar_repository
            .read(uow, command.currency_registrar_id)
            .await?;

        let unique_value =
            Self::registrar_user_unique_value(command.currency_registrar_id, command.user_id)?;
        let mut membership = CurrencyRegistrarMembership::new();
        let currency_registrar_membership_id = membership.aggregate_id();
        if self
            .currency_registrar_membership_repository
            .find_by_unique_value(
                uow,
                CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY,
                &unique_value,
            )
            .await?
            .is_some()
        {
            let reason = CurrencyRegistrarMembershipCreateRejectionReason::AlreadyMember;
            membership.reject_create(command.currency_registrar_id, command.user_id, reason)?;
            self.currency_registrar_membership_repository
                .save(uow, request_context, &mut membership)
                .await?;
            return Ok(CurrencyRegistrarMembershipCreateOutput::Rejected {
                currency_registrar_membership_id,
                reason,
            });
        }

        let result = membership.create(command.currency_registrar_id, command.user_id)?;
        self.currency_registrar_membership_repository
            .save(uow, request_context, &mut membership)
            .await?;
        Ok(match result {
            CurrencyRegistrarMembershipCreateResult::Created => {
                CurrencyRegistrarMembershipCreateOutput::Created {
                    currency_registrar_membership_id,
                }
            }
            CurrencyRegistrarMembershipCreateResult::Rejected { reason } => {
                CurrencyRegistrarMembershipCreateOutput::Rejected {
                    currency_registrar_membership_id,
                    reason,
                }
            }
        })
    }
}
