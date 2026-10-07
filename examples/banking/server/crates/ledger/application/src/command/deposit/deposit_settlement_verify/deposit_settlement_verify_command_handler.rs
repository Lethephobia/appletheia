use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{Repository, RepositoryError};
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::deposit::Deposit;
use banking_ledger_domain::deposit::DepositError;
use banking_ledger_domain::token_binding::TokenBinding;

use crate::settlement::{DepositSettlementVerifier, DepositSettlementVerifyRequest};

use super::{
    DepositSettlementVerifyCommand, DepositSettlementVerifyCommandHandlerError,
    DepositSettlementVerifyOutput,
};

pub struct DepositSettlementVerifyCommandHandler<R, TDV>
where
    R: Repository,
    TDV: DepositSettlementVerifier,
{
    repository: R,
    deposit_settlement_verifier: TDV,
}

impl<R, TDV> DepositSettlementVerifyCommandHandler<R, TDV>
where
    R: Repository,
    TDV: DepositSettlementVerifier,
{
    pub fn new(repository: R, deposit_settlement_verifier: TDV) -> Self {
        Self {
            repository,
            deposit_settlement_verifier,
        }
    }
}

impl<R, TDV> CommandHandler for DepositSettlementVerifyCommandHandler<R, TDV>
where
    R: Repository,
    TDV: DepositSettlementVerifier,
{
    type Command = DepositSettlementVerifyCommand;
    type Output = DepositSettlementVerifyOutput;
    type Error = DepositSettlementVerifyCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::Authenticated,
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut deposit = self
            .repository
            .read::<Deposit>(uow, command.deposit_id)
            .await?;
        let account = self
            .repository
            .read::<Account>(uow, *deposit.account_id()?)
            .await?;
        let currency = self
            .repository
            .read::<Currency>(uow, *account.currency_id()?)
            .await?;
        let token_binding = match self
            .repository
            .read::<TokenBinding>(uow, deposit.token_binding_id()?)
            .await
        {
            Ok(token_binding)
                if token_binding.is_active()?
                    && token_binding.is_deposit_enabled()?
                    && token_binding.currency_id()? == *account.currency_id()? =>
            {
                token_binding
            }
            Ok(_) | Err(RepositoryError::NotFound { .. }) => {
                return Err(DepositError::TokenBindingUnavailable.into());
            }
            Err(error) => return Err(error.into()),
        };
        let chain_network = token_binding.chain_network()?;
        if !command.transaction_id.matches_network(chain_network) {
            return Err(DepositError::ChainMismatch.into());
        }

        let verification = self
            .deposit_settlement_verifier
            .verify(DepositSettlementVerifyRequest {
                deposit_id: command.deposit_id,
                currency_decimals: currency.decimals()?,
                chain_network,
                token_address: *token_binding.token_address()?,
                token_owner_address: *deposit.token_owner_address()?,
                amount: deposit.amount()?,
                transaction_id: command.transaction_id,
            })
            .await?;
        deposit.record_settlement_verified(verification.transaction_id)?;
        self.repository
            .save(uow, request_context, &mut deposit)
            .await?;

        Ok(DepositSettlementVerifyOutput {})
    }
}
