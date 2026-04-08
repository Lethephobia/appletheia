use appletheia::aggregate_state;
use appletheia::unique_constraints;

use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency_definition::CurrencyDefinitionId;

use super::{CurrencyIssuanceId, CurrencyIssuanceStateError, CurrencyIssuanceStatus};

/// Stores the materialized state of a `CurrencyIssuance` aggregate.
#[aggregate_state(error = CurrencyIssuanceStateError)]
#[unique_constraints()]
pub struct CurrencyIssuanceState {
    pub(super) id: CurrencyIssuanceId,
    pub(super) currency_definition_id: CurrencyDefinitionId,
    pub(super) destination_account_id: AccountId,
    pub(super) amount: CurrencyAmount,
    pub(super) status: CurrencyIssuanceStatus,
}

impl CurrencyIssuanceState {
    /// Creates a new issuance state.
    pub(super) fn new(
        id: CurrencyIssuanceId,
        currency_definition_id: CurrencyDefinitionId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            id,
            currency_definition_id,
            destination_account_id,
            amount,
            status: CurrencyIssuanceStatus::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateState;

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;
    use crate::currency_definition::CurrencyDefinitionId;

    use super::{CurrencyIssuanceId, CurrencyIssuanceState, CurrencyIssuanceStatus};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = CurrencyIssuanceId::new();
        let state = CurrencyIssuanceState::new(
            id,
            CurrencyDefinitionId::new(),
            AccountId::new(),
            CurrencyAmount::new(1),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.status, CurrencyIssuanceStatus::Pending);
    }
}
