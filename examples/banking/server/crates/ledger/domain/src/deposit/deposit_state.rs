use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

use super::{DepositId, DepositStateError, DepositStatus};

/// Stores the materialized state of a `Deposit` aggregate.
#[aggregate_state(error = DepositStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "account", value = account_ref_value),
    entry(key = "currency", value = currency_ref_value)
)]
pub struct DepositState {
    pub(super) id: DepositId,
    pub(super) account_id: AccountId,
    pub(super) currency_id: CurrencyId,
    pub(super) token_account_owner_address: TokenAccountOwnerAddress,
    pub(super) amount: CurrencyAmount,
    pub(super) status: DepositStatus,
}

fn account_ref_value(state: &DepositState) -> Result<Option<AccountId>, DepositStateError> {
    Ok(Some(state.account_id))
}

fn currency_ref_value(state: &DepositState) -> Result<Option<CurrencyId>, DepositStateError> {
    Ok(Some(state.currency_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{ReferenceIndexes, ReferenceValues};

    use crate::account::AccountId;
    use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
    use crate::currency::CurrencyId;

    use super::{DepositId, DepositState, DepositStatus};

    #[test]
    fn returns_reference_entries_for_account_and_currency() {
        let state = deposit_state();

        let entries = state
            .reference_entries()
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(DepositState::ACCOUNT_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
        assert_eq!(
            entries
                .get(DepositState::CURRENCY_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }

    fn deposit_state() -> DepositState {
        DepositState {
            id: DepositId::new(),
            account_id: AccountId::new(),
            currency_id: CurrencyId::new(),
            token_account_owner_address: TokenAccountOwnerAddress::try_from(
                "11111111111111111111111111111111",
            )
            .expect("token account owner address should be valid"),
            amount: CurrencyAmount::new(100),
            status: DepositStatus::Requested,
        }
    }
}
