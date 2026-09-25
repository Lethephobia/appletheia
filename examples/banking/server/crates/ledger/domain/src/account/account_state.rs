use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::{OrganizationId, UserId};
use uuid::Uuid;

use crate::currency::CurrencyId;

use super::{
    AccountBalance, AccountDescription, AccountName, AccountOwner, AccountStateError, AccountStatus,
};

/// Stores the materialized state of an `Account` aggregate.
#[aggregate_state(error = AccountStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "owner_user", value = owner_user_ref_value),
    entry(key = "owner_organization", value = owner_organization_ref_value),
    entry(key = "currency", value = currency_ref_value)
)]
pub struct AccountState {
    pub(super) owner: AccountOwner,
    pub(super) name: AccountName,
    pub(super) description: Option<AccountDescription>,
    pub(super) currency_id: CurrencyId,
    pub(super) balance: AccountBalance,
    pub(super) status: AccountStatus,
}

fn owner_user_ref_value(
    state: &AccountState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, AccountStateError> {
    Ok(state.owner.user_id().copied())
}

fn owner_organization_ref_value(
    state: &AccountState,
    _aggregate_id: Uuid,
) -> Result<Option<OrganizationId>, AccountStateError> {
    Ok(state.owner.organization_id().copied())
}

fn currency_ref_value(
    state: &AccountState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyId>, AccountStateError> {
    Ok(Some(state.currency_id))
}
