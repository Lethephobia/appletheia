use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::Currency;
use banking_ledger_domain::payout_destination::PayoutDestination;
use banking_ledger_domain::withdrawal::{
    Withdrawal, WithdrawalOnchainTransactionId, WithdrawalTokenTransferResult,
};

use crate::banking_ledger::{
    MintAccountAddress, MintAccountDecimals, PoolTokenAccountAddress, PoolTokenTransferExecutor,
    PoolTokenTransferMarkerSeed, PoolTokenTransferRequest, TokenAccountOwnerAddress, TokenAmount,
};

use super::{
    WithdrawalTokenTransferCommand, WithdrawalTokenTransferCommandHandlerError,
    WithdrawalTokenTransferOutput,
};

/// Handles `WithdrawalTokenTransferCommand`.
pub struct WithdrawalTokenTransferCommandHandler<WR, PDR, CR, PTTE>
where
    WR: Repository<Withdrawal>,
    PDR: Repository<PayoutDestination, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PTTE: PoolTokenTransferExecutor,
{
    withdrawal_repository: WR,
    payout_destination_repository: PDR,
    currency_repository: CR,
    pool_token_transfer_executor: PTTE,
}

impl<WR, PDR, CR, PTTE> WithdrawalTokenTransferCommandHandler<WR, PDR, CR, PTTE>
where
    WR: Repository<Withdrawal>,
    PDR: Repository<PayoutDestination, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PTTE: PoolTokenTransferExecutor,
{
    pub fn new(
        withdrawal_repository: WR,
        payout_destination_repository: PDR,
        currency_repository: CR,
        pool_token_transfer_executor: PTTE,
    ) -> Self {
        Self {
            withdrawal_repository,
            payout_destination_repository,
            currency_repository,
            pool_token_transfer_executor,
        }
    }
}

impl<WR, PDR, CR, PTTE> CommandHandler for WithdrawalTokenTransferCommandHandler<WR, PDR, CR, PTTE>
where
    WR: Repository<Withdrawal>,
    PDR: Repository<PayoutDestination, Uow = WR::Uow>,
    CR: Repository<Currency, Uow = WR::Uow>,
    PTTE: PoolTokenTransferExecutor,
{
    type Command = WithdrawalTokenTransferCommand;
    type Output = WithdrawalTokenTransferOutput;
    type ReplayOutput = WithdrawalTokenTransferOutput;
    type Error = WithdrawalTokenTransferCommandHandlerError;
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
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let mut withdrawal = self
            .withdrawal_repository
            .read(uow, command.withdrawal_id)
            .await?;
        let payout_destination = self
            .payout_destination_repository
            .read(uow, *withdrawal.payout_destination_id()?)
            .await?;
        let currency = self
            .currency_repository
            .read(uow, *withdrawal.currency_id()?)
            .await?;
        let Some(mint_account) = currency.mint_account()? else {
            return Err(WithdrawalTokenTransferCommandHandlerError::CurrencyUnprovisioned);
        };

        let request = PoolTokenTransferRequest::new(
            PoolTokenTransferMarkerSeed::try_from(command.withdrawal_id)?,
            MintAccountAddress::try_from(mint_account.mint_account_address().value())?,
            PoolTokenAccountAddress::try_from(mint_account.pool_token_account_address().value())?,
            TokenAccountOwnerAddress::try_from(
                payout_destination.token_account_owner_address()?.value(),
            )?,
            TokenAmount::new(withdrawal.amount()?.value()),
            MintAccountDecimals::from(currency.decimals()?),
        );

        let receipt = self.pool_token_transfer_executor.execute(request).await?;
        let onchain_transaction_id = WithdrawalOnchainTransactionId::new(
            receipt.onchain_transaction_id().value().to_owned(),
        )
        .ok_or(WithdrawalTokenTransferCommandHandlerError::InvalidOnchainTransactionId)?;
        let result = withdrawal.record_token_transfer(onchain_transaction_id.clone())?;
        self.withdrawal_repository
            .save(uow, request_context, &mut withdrawal)
            .await?;
        let output = match result {
            WithdrawalTokenTransferResult::TokenTransferred => {
                WithdrawalTokenTransferOutput::TokenTransferred {
                    onchain_transaction_id,
                }
            }
            WithdrawalTokenTransferResult::Rejected { .. } => {
                WithdrawalTokenTransferOutput::Rejected
            }
        };

        Ok(CommandHandled::same(output))
    }
}
