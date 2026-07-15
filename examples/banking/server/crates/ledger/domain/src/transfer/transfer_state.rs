use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::account::AccountId;
use crate::core::CurrencyAmount;

use super::{TransferStateError, TransferStatus};

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

#[cfg(test)]
mod tests {
    use appletheia::domain::{ReferenceIndexes, ReferenceValues};
    use uuid::Uuid;

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;

    use super::{TransferState, TransferStatus};

    #[test]
    fn state_stores_domain_attributes() {
        let state = TransferState {
            from_account_id: AccountId::new(),
            to_account_id: AccountId::new(),
            amount: CurrencyAmount::new(1),
            status: TransferStatus::Pending,
        };
        assert_eq!(state.status, TransferStatus::Pending);
    }

    #[test]
    fn returns_reference_entries_for_source_and_destination_accounts() {
        let state = TransferState {
            from_account_id: AccountId::new(),
            to_account_id: AccountId::new(),
            amount: CurrencyAmount::new(1),
            status: TransferStatus::Pending,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(TransferState::FROM_ACCOUNT_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(TransferState::TO_ACCOUNT_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
