use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, OnchainTransactionId};
use crate::token_binding::TokenBindingId;

use super::{DepositNote, DepositStateError, DepositStatus};

/// Stores the materialized state of a `Deposit` aggregate.
#[aggregate_state(error = DepositStateError)]
#[unique_constraints()]
#[reference_indexes(entry(key = "account", value = account_ref_value))]
pub struct DepositState {
    pub(super) account_id: AccountId,
    pub(super) token_binding_id: TokenBindingId,
    pub(super) amount: CurrencyAmount,
    pub(super) note: Option<DepositNote>,
    pub(super) transaction_id: Option<OnchainTransactionId>,
    pub(super) status: DepositStatus,
}

fn account_ref_value(
    state: &DepositState,
    _aggregate_id: Uuid,
) -> Result<Option<AccountId>, DepositStateError> {
    Ok(Some(state.account_id))
}
