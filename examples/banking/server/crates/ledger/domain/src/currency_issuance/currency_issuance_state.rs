use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency::CurrencyId;

use super::{CurrencyIssuanceStateError, CurrencyIssuanceStatus};

/// Stores the materialized state of a `CurrencyIssuance` aggregate.
#[aggregate_state(error = CurrencyIssuanceStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "currency", value = currency_ref_value),
    entry(key = "destination_account", value = destination_account_ref_value)
)]
pub struct CurrencyIssuanceState {
    pub(super) currency_id: CurrencyId,
    pub(super) destination_account_id: AccountId,
    pub(super) amount: CurrencyAmount,
    pub(super) status: CurrencyIssuanceStatus,
}

fn currency_ref_value(
    state: &CurrencyIssuanceState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyId>, CurrencyIssuanceStateError> {
    Ok(Some(state.currency_id))
}

fn destination_account_ref_value(
    state: &CurrencyIssuanceState,
    _aggregate_id: Uuid,
) -> Result<Option<AccountId>, CurrencyIssuanceStateError> {
    Ok(Some(state.destination_account_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{ReferenceIndexes, ReferenceValues};
    use uuid::Uuid;

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;
    use crate::currency::CurrencyId;

    use super::{CurrencyIssuanceState, CurrencyIssuanceStatus};

    #[test]
    fn state_stores_domain_attributes() {
        let state = CurrencyIssuanceState {
            currency_id: CurrencyId::new(),
            destination_account_id: AccountId::new(),
            amount: CurrencyAmount::new(1),
            status: CurrencyIssuanceStatus::Pending,
        };
        assert_eq!(state.status, CurrencyIssuanceStatus::Pending);
    }

    #[test]
    fn returns_reference_entries_for_currency_and_destination_account() {
        let state = CurrencyIssuanceState {
            currency_id: CurrencyId::new(),
            destination_account_id: AccountId::new(),
            amount: CurrencyAmount::new(1),
            status: CurrencyIssuanceStatus::Pending,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
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
