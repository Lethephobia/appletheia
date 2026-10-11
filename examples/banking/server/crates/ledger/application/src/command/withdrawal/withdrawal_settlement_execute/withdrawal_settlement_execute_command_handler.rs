use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::withdrawal::WithdrawalError;
use banking_ledger_domain::{
    account::Account, currency::Currency, token_binding::TokenBinding, withdrawal::Withdrawal,
};

use super::{
    WithdrawalSettlementExecuteCommand, WithdrawalSettlementExecuteCommandHandlerError,
    WithdrawalSettlementExecuteOutput,
};
use crate::settlement::{WithdrawalSettlementExecutor, WithdrawalSettlementRequest};

pub struct WithdrawalSettlementExecuteCommandHandler<R, WSE>
where
    R: Repository,
    WSE: WithdrawalSettlementExecutor,
{
    repository: R,
    withdrawal_settlement_executor: WSE,
}

impl<R, WSE> WithdrawalSettlementExecuteCommandHandler<R, WSE>
where
    R: Repository,
    WSE: WithdrawalSettlementExecutor,
{
    pub fn new(repository: R, withdrawal_settlement_executor: WSE) -> Self {
        Self {
            repository,
            withdrawal_settlement_executor,
        }
    }
}

impl<R, WSE> CommandHandler for WithdrawalSettlementExecuteCommandHandler<R, WSE>
where
    R: Repository,
    WSE: WithdrawalSettlementExecutor,
{
    type Command = WithdrawalSettlementExecuteCommand;
    type Output = WithdrawalSettlementExecuteOutput;
    type Error = WithdrawalSettlementExecuteCommandHandlerError;
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
        let mut withdrawal = self
            .repository
            .read::<Withdrawal>(uow, command.withdrawal_id)
            .await?;
        let account = self
            .repository
            .read_shared::<Account>(uow, *withdrawal.account_id()?)
            .await?;
        let currency = self
            .repository
            .read_shared::<Currency>(uow, *account.currency_id()?)
            .await?;
        let token_binding_id = withdrawal.token_binding_id()?;
        let token_binding = match self
            .repository
            .read_shared::<TokenBinding>(uow, token_binding_id)
            .await
        {
            Ok(token_binding)
                if token_binding.is_active()? && token_binding.is_withdrawal_enabled()? =>
            {
                token_binding
            }
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                return Err(WithdrawalError::TokenBindingUnavailable.into());
            }
            Err(error) => return Err(error.into()),
        };
        if token_binding.currency_id()? != *account.currency_id()? {
            return Err(WithdrawalError::TokenBindingUnavailable.into());
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

        withdrawal.record_settlement_executed(execution.transaction_id)?;
        self.repository
            .save(uow, request_context, &mut withdrawal)
            .await?;

        Ok(WithdrawalSettlementExecuteOutput {})
    }
}
