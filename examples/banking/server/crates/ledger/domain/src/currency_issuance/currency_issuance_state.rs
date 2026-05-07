use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{CurrencyIssuanceId, CurrencyIssuanceStateError, CurrencyIssuanceStatus};

/// Stores the materialized state of a `CurrencyIssuance` aggregate.
#[aggregate_state(error = CurrencyIssuanceStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "currency", value = currency_value),
    entry(key = "destination_account", value = destination_account_value)
)]
pub struct CurrencyIssuanceState {
    pub(super) id: CurrencyIssuanceId,
    pub(super) currency_id: CurrencyId,
    pub(super) destination_account_id: AccountId,
    pub(super) amount: CurrencyAmount,
    pub(super) status: CurrencyIssuanceStatus,
}

impl CurrencyIssuanceState {
    /// Creates a new issuance state.
    pub(super) fn new(
        id: CurrencyIssuanceId,
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            id,
            currency_id,
            destination_account_id,
            amount,
            status: CurrencyIssuanceStatus::Pending,
        }
    }

    /// Creates a new rejected issuance state.
    pub(super) fn rejected(
        id: CurrencyIssuanceId,
        currency_id: CurrencyId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            id,
            currency_id,
            destination_account_id,
            amount,
            status: CurrencyIssuanceStatus::Rejected,
        }
    }
}

fn currency_value(
    state: &CurrencyIssuanceState,
) -> Result<Option<CurrencyId>, CurrencyIssuanceStateError> {
    Ok(Some(state.currency_id))
}

fn destination_account_value(
    state: &CurrencyIssuanceState,
) -> Result<Option<AccountId>, CurrencyIssuanceStateError> {
    Ok(Some(state.destination_account_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, ReferenceIndexes, ReferenceValues};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;
    use crate::currency::CurrencyId;

    use super::{CurrencyIssuanceId, CurrencyIssuanceState, CurrencyIssuanceStatus};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = CurrencyIssuanceId::new();
        let state = CurrencyIssuanceState::new(
            id,
            CurrencyId::new(),
            AccountId::new(),
            CurrencyAmount::new(1),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.status, CurrencyIssuanceStatus::Pending);
    }

    #[test]
    fn returns_reference_entries_for_currency_and_destination_account() {
        let state = CurrencyIssuanceState::new(
            CurrencyIssuanceId::new(),
            CurrencyId::new(),
            AccountId::new(),
            CurrencyAmount::new(1),
        );

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(CurrencyIssuanceState::CURRENCY_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(CurrencyIssuanceState::DESTINATION_ACCOUNT_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
