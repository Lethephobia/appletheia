use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::{OrganizationId, UserId};

use crate::core::TokenAccountOwnerAddress;

use super::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
    WalletBookmarkStateError, WalletBookmarkStatus,
};

/// Stores the materialized state of a `WalletBookmark` aggregate.
#[aggregate_state(error = WalletBookmarkStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "user_owner", value = user_owner_ref_value),
    entry(key = "organization_owner", value = organization_owner_ref_value)
)]
pub struct WalletBookmarkState {
    pub(super) id: WalletBookmarkId,
    pub(super) owner: WalletBookmarkOwner,
    pub(super) display_name: Option<WalletBookmarkDisplayName>,
    pub(super) description: Option<WalletBookmarkDescription>,
    pub(super) token_account_owner_address: TokenAccountOwnerAddress,
    pub(super) status: WalletBookmarkStatus,
}

fn user_owner_ref_value(
    state: &WalletBookmarkState,
) -> Result<Option<UserId>, WalletBookmarkStateError> {
    Ok(state.owner.user_id().copied())
}

fn organization_owner_ref_value(
    state: &WalletBookmarkState,
) -> Result<Option<OrganizationId>, WalletBookmarkStateError> {
    Ok(state.owner.organization_id().copied())
}
