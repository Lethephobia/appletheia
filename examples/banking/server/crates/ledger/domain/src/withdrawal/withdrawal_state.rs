use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId, TokenOwnerAddress};
use crate::token_binding::TokenBindingId;

use super::{WithdrawalNote, WithdrawalStateError, WithdrawalStatus};

/// Stores the materialized state of a `Withdrawal` aggregate.
#[aggregate_state(error = WithdrawalStateError)]
#[unique_constraints()]
#[reference_indexes(entry(key = "account", value = account_ref_value))]
pub struct WithdrawalState {
    pub(super) account_id: AccountId,
    pub(super) token_binding_id: TokenBindingId,
    pub(super) token_owner_address: TokenOwnerAddress,
    pub(super) amount: CurrencyAmount,
    pub(super) note: Option<WithdrawalNote>,
    pub(super) transaction_id: Option<OnchainTransactionId>,
    pub(super) status: WithdrawalStatus,
}

fn account_ref_value(
    state: &WithdrawalState,
    _aggregate_id: Uuid,
) -> Result<Option<AccountId>, WithdrawalStateError> {
    Ok(Some(state.account_id))
}
