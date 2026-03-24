use appletheia::aggregate_state;
use appletheia::domain::UniqueConstraints;
use banking_iam_domain::UserId;

use crate::currency_definition::CurrencyDefinitionId;

use super::{AccountBalance, AccountId, AccountStateError};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
pub struct AccountState {
    pub(super) id: AccountId,
    pub(super) user_id: UserId,
    pub(super) currency_definition_id: CurrencyDefinitionId,
    pub(super) balance: AccountBalance,
    pub(super) reserved_balance: AccountBalance,
    pub(super) frozen: bool,
}

impl AccountState {
    /// Creates a new account state.
    pub(super) fn new(
        id: AccountId,
        user_id: UserId,
        currency_definition_id: CurrencyDefinitionId,
    ) -> Self {
        Self {
            id,
            user_id,
            currency_definition_id,
            balance: AccountBalance::zero(),
            reserved_balance: AccountBalance::zero(),
            frozen: false,
        }
    }
}

impl UniqueConstraints<AccountStateError> for AccountState {}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateState;

    use banking_iam_domain::UserId;

    use crate::currency_definition::CurrencyDefinitionId;

    use super::{AccountBalance, AccountId, AccountState};

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = AccountId::new();
        let state = AccountState::new(id, UserId::new(), CurrencyDefinitionId::new());

        assert_eq!(state.id(), id);
    }

    #[test]
    fn new_initializes_zero_balances_and_not_frozen() {
        let state = AccountState::new(AccountId::new(), UserId::new(), CurrencyDefinitionId::new());

        assert_eq!(state.balance, AccountBalance::zero());
        assert_eq!(state.reserved_balance, AccountBalance::zero());
        assert!(!state.frozen);
    }
}
