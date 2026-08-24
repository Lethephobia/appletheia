use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{TransferNote, TransferStateError, TransferStatus};

/// Stores the materialized state of a `Transfer` aggregate.
#[aggregate_state(error = TransferStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "from_account", value = from_account_ref_value),
    entry(key = "to_account", value = to_account_ref_value)
)]
pub struct TransferState {
    pub(super) from_account_id: AccountId,
    pub(super) to_account_id: AccountId,
    pub(super) amount: CurrencyAmount,
    pub(super) note: Option<TransferNote>,
    pub(super) status: TransferStatus,
}

fn from_account_ref_value(
    state: &TransferState,
    _aggregate_id: Uuid,
) -> Result<Option<AccountId>, TransferStateError> {
    Ok(Some(state.from_account_id))
}

fn to_account_ref_value(
    state: &TransferState,
    _aggregate_id: Uuid,
) -> Result<Option<AccountId>, TransferStateError> {
    Ok(Some(state.to_account_id))
}
