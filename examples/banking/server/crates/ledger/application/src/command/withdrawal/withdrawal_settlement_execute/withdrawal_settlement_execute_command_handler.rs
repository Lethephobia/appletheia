use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::{
    account::Account,
    currency::Currency,
    token_binding::TokenBinding,
    withdrawal::{
        Withdrawal, WithdrawalSettlementExecuteRejectionReason, WithdrawalSettlementExecuteResult,
    },
};

use super::{
    WithdrawalSettlementExecuteCommand, WithdrawalSettlementExecuteCommandHandlerError,
    WithdrawalSettlementExecuteOutput,
};
use crate::settlement::{WithdrawalSettlementExecutor, WithdrawalSettlementRequest};

pub struct WithdrawalSettlementExecuteCommandHandler<WR, AR, CR, TBR, WSE>
where
    WR: Repository<Withdrawal>,
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    TBR: Repository<TokenBinding, Uow = WR::Uow>,
    WSE: WithdrawalSettlementExecutor,
{
    withdrawal_repository: WR,
    account_repository: AR,
    currency_repository: CR,
    token_binding_repository: TBR,
    withdrawal_settlement_executor: WSE,
}

impl<WR, AR, CR, TBR, WSE> WithdrawalSettlementExecuteCommandHandler<WR, AR, CR, TBR, WSE>
where
    WR: Repository<Withdrawal>,
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    TBR: Repository<TokenBinding, Uow = WR::Uow>,
    WSE: WithdrawalSettlementExecutor,
{
    pub fn new(
        withdrawal_repository: WR,
        account_repository: AR,
        currency_repository: CR,
        token_binding_repository: TBR,
        withdrawal_settlement_executor: WSE,
    ) -> Self {
        Self {
            withdrawal_repository,
            account_repository,
            currency_repository,
            token_binding_repository,
            withdrawal_settlement_executor,
        }
    }
}

impl<WR, AR, CR, TBR, WSE> CommandHandler
    for WithdrawalSettlementExecuteCommandHandler<WR, AR, CR, TBR, WSE>
where
    WR: Repository<Withdrawal>,
    AR: Repository<Account, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    TBR: Repository<TokenBinding, Uow = WR::Uow>,
    WSE: WithdrawalSettlementExecutor,
{
    type Command = WithdrawalSettlementExecuteCommand;
    type Output = WithdrawalSettlementExecuteOutput;
    type Error = WithdrawalSettlementExecuteCommandHandlerError;
    type Uow = WR::Uow;

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
        let mut withdrawal = self
            .withdrawal_repository
            .read(uow, command.withdrawal_id)
            .await?;
        let account = self
            .account_repository
            .read(uow, *withdrawal.account_id()?)
            .await?;
        let currency = self
            .currency_repository
            .read(uow, *account.currency_id()?)
            .await?;
        let token_binding_id = withdrawal.token_binding_id()?;
        let token_binding = match self
            .token_binding_repository
            .read(uow, token_binding_id)
            .await
        {
            Ok(token_binding) if token_binding.is_active()? => token_binding,
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                let reason = WithdrawalSettlementExecuteRejectionReason::TokenBindingUnavailable;
                withdrawal.reject_settlement_execute(None, reason)?;
                self.withdrawal_repository
                    .save(uow, request_context, &mut withdrawal)
                    .await?;
                return Ok(WithdrawalSettlementExecuteOutput::Rejected { reason });
            }
            Err(error) => return Err(error.into()),
        };
        if token_binding.currency_id()? != *account.currency_id()? {
            let reason = WithdrawalSettlementExecuteRejectionReason::TokenBindingUnavailable;
            withdrawal.reject_settlement_execute(None, reason)?;
            self.withdrawal_repository
                .save(uow, request_context, &mut withdrawal)
                .await?;
            return Ok(WithdrawalSettlementExecuteOutput::Rejected { reason });
        }
        let chain_network = token_binding.chain_network()?;
        let execution = self
            .withdrawal_settlement_executor
            .execute(WithdrawalSettlementRequest::new(
                command.withdrawal_id,
                currency.decimals()?,
                chain_network,
                *token_binding.token_address()?,
                *withdrawal.token_owner_address()?,
                withdrawal.amount()?,
            ))
            .await?;

        let result = withdrawal.record_settlement_executed(execution.transaction_id)?;
        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(match result {
            WithdrawalSettlementExecuteResult::Executed => {
                WithdrawalSettlementExecuteOutput::Executed
            }
            WithdrawalSettlementExecuteResult::Rejected { reason } => {
                WithdrawalSettlementExecuteOutput::Rejected { reason }
            }
        })
    }
}
