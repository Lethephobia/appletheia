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
        let state = CurrencyIssuanceState {
            id,
            currency_id: CurrencyId::new(),
            destination_account_id: AccountId::new(),
            amount: CurrencyAmount::new(1),
            status: CurrencyIssuanceStatus::Pending,
        };

        assert_eq!(state.id(), id);
        assert_eq!(state.status, CurrencyIssuanceStatus::Pending);
    }

    #[test]
    fn returns_reference_entries_for_currency_and_destination_account() {
        let state = CurrencyIssuanceState {
            id: CurrencyIssuanceId::new(),
            currency_id: CurrencyId::new(),
            destination_account_id: AccountId::new(),
            amount: CurrencyAmount::new(1),
            status: CurrencyIssuanceStatus::Pending,
        };

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
