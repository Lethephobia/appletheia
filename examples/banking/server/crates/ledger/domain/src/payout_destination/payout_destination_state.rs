use appletheia::aggregate_state;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::{OrganizationId, UserId};

use super::{
    PayoutDestinationId, PayoutDestinationOwner, PayoutDestinationStateError,
    PayoutDestinationStatus, TokenAccountOwnerAddress,
};

/// Stores the materialized state of a `PayoutDestination` aggregate.
#[aggregate_state(error = PayoutDestinationStateError)]
#[unique_constraints()]
#[reference_indexes(
    entry(key = "user_owner", value = user_owner_ref_value),
    entry(key = "organization_owner", value = organization_owner_ref_value)
)]
pub struct PayoutDestinationState {
    pub(super) id: PayoutDestinationId,
    pub(super) owner: PayoutDestinationOwner,
    pub(super) token_account_owner_address: TokenAccountOwnerAddress,
    pub(super) status: PayoutDestinationStatus,
}

fn user_owner_ref_value(
    state: &PayoutDestinationState,
) -> Result<Option<UserId>, PayoutDestinationStateError> {
    Ok(state.owner.user_id().copied())
}

fn organization_owner_ref_value(
    state: &PayoutDestinationState,
) -> Result<Option<OrganizationId>, PayoutDestinationStateError> {
    Ok(state.owner.organization_id().copied())
}
