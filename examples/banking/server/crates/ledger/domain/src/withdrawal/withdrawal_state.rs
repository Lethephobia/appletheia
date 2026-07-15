use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::account::AccountId;
use crate::core::{CurrencyAmount, TokenAccountOwnerAddress};
use crate::currency::CurrencyId;

use super::{WithdrawalStateError, WithdrawalStatus};

/// Stores the materialized state of a `Withdrawal` aggregate.
#[aggregate_state(error = WithdrawalStateError)]
#[unique_constraints()]
#[reference_indexes(entry(key = "account", value = account_ref_value))]
pub struct WithdrawalState {
    pub(super) account_id: AccountId,
    pub(super) currency_id: CurrencyId,
    pub(super) token_account_owner_address: TokenAccountOwnerAddress,
    pub(super) amount: CurrencyAmount,
    pub(super) status: WithdrawalStatus,
}

fn account_ref_value(
    state: &WithdrawalState,
    _aggregate_id: Uuid,
) -> Result<Option<AccountId>, WithdrawalStateError> {
    Ok(Some(state.account_id))
}
