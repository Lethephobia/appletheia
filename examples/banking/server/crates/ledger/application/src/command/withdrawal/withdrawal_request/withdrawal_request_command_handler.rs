use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::token_binding::TokenBinding;
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalRequest, WithdrawalRequestRejectionReason, WithdrawalRequestResult,
};

use super::{
    WithdrawalRequestCommand, WithdrawalRequestCommandHandlerError, WithdrawalRequestOutput,
};
use crate::authorization::AccountWithdrawalRequesterRelation;

pub struct WithdrawalRequestCommandHandler<WR, AR, TBR>
where
    WR: Repository<Withdrawal>,
    AR: Repository<Account, Uow = WR::Uow>,
    TBR: Repository<TokenBinding, Uow = WR::Uow>,
{
    withdrawal_repository: WR,
    account_repository: AR,
    token_binding_repository: TBR,
}

impl<WR, AR, TBR> WithdrawalRequestCommandHandler<WR, AR, TBR>
where
    WR: Repository<Withdrawal>,
    AR: Repository<Account, Uow = WR::Uow>,
    TBR: Repository<TokenBinding, Uow = WR::Uow>,
{
    pub fn new(
        withdrawal_repository: WR,
        account_repository: AR,
        token_binding_repository: TBR,
    ) -> Self {
        Self {
            withdrawal_repository,
            account_repository,
            token_binding_repository,
        }
    }
}

impl<WR, AR, TBR> CommandHandler for WithdrawalRequestCommandHandler<WR, AR, TBR>
where
    WR: Repository<Withdrawal>,
    AR: Repository<Account, Uow = WR::Uow>,
    TBR: Repository<TokenBinding, Uow = WR::Uow>,
{
    type Command = WithdrawalRequestCommand;
    type Output = WithdrawalRequestOutput;
    type Error = WithdrawalRequestCommandHandlerError;
    type Uow = WR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Account,
            >(
                command.account_id,
                AccountWithdrawalRequesterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut withdrawal = Withdrawal::new();
        let withdrawal_id = withdrawal.aggregate_id();
        let request = WithdrawalRequest {
            account_id: command.account_id,
            token_binding_id: command.token_binding_id,
            token_owner_address: command.token_owner_address,
            amount: command.amount,
            note: command.note.clone(),
        };
        let account = self
            .account_repository
            .read(uow, command.account_id)
            .await?;
        match self
            .token_binding_repository
            .read(uow, command.token_binding_id)
            .await
        {
            Ok(token_binding)
                if token_binding.is_active()?
                    && token_binding.currency_id()? == *account.currency_id()? => {}
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                let reason = WithdrawalRequestRejectionReason::TokenBindingUnavailable;
                withdrawal.reject_request(request, reason)?;
                self.withdrawal_repository
                    .save(uow, request_context, &mut withdrawal)
                    .await?;
                return Ok(WithdrawalRequestOutput::Rejected {
                    withdrawal_id,
                    reason,
                });
            }
            Err(error) => return Err(error.into()),
        }
        let result = withdrawal.request(request)?;

        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(match result {
            WithdrawalRequestResult::Requested => {
                WithdrawalRequestOutput::Requested { withdrawal_id }
            }
            WithdrawalRequestResult::Rejected { reason } => WithdrawalRequestOutput::Rejected {
                withdrawal_id,
                reason,
            },
        })
    }
}
