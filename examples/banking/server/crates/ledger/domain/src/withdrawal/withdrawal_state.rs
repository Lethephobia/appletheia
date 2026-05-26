use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::{
    account::AccountId, core::CurrencyAmount, currency::CurrencyId,
    payout_destination::PayoutDestinationId,
};

use super::{WithdrawalId, WithdrawalOnchainTransactionId, WithdrawalStateError, WithdrawalStatus};

/// Stores the materialized state of a `Withdrawal` aggregate.
#[aggregate_state(error = WithdrawalStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "account", value = account_ref_value),
    entry(key = "payout_destination", value = payout_destination_ref_value)
)]
pub struct WithdrawalState {
    pub(super) id: WithdrawalId,
    pub(super) account_id: AccountId,
    pub(super) currency_id: CurrencyId,
    pub(super) payout_destination_id: PayoutDestinationId,
    pub(super) amount: CurrencyAmount,
    pub(super) onchain_transaction_id: Option<WithdrawalOnchainTransactionId>,
    pub(super) status: WithdrawalStatus,
}

fn account_ref_value(state: &WithdrawalState) -> Result<Option<AccountId>, WithdrawalStateError> {
    Ok(Some(state.account_id))
}

fn payout_destination_ref_value(
    state: &WithdrawalState,
) -> Result<Option<PayoutDestinationId>, WithdrawalStateError> {
    Ok(Some(state.payout_destination_id))
}
