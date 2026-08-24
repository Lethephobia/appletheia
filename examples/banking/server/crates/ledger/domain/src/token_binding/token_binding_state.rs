use appletheia::aggregate_state;
use appletheia::domain::UniqueValue;
use appletheia::{reference_indexes, unique_constraints};
use uuid::Uuid;

use crate::core::{ChainNetwork, TokenAddress};
use crate::currency::CurrencyId;

use super::{TokenBindingStateError, TokenBindingStatus};

#[aggregate_state(error = TokenBindingStateError)]
#[unique_constraints(entry(key = "token", value = token_unique_value))]
#[reference_indexes(entry(key = "currency", value = currency_ref_value))]
pub struct TokenBindingState {
    pub(super) currency_id: CurrencyId,
    pub(super) chain_network: ChainNetwork,
    pub(super) token_address: TokenAddress,
    pub(super) deposit_enabled: bool,
    pub(super) withdrawal_enabled: bool,
    pub(super) status: TokenBindingStatus,
}

fn token_unique_value(
    state: &TokenBindingState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, TokenBindingStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }
    let network_name = state.chain_network.network_name();
    let token_address = state.token_address.to_string();
    Ok(Some(UniqueValue::from_strings([
        state.chain_network.chain_name(),
        network_name.as_str(),
        token_address.as_str(),
    ])?))
}

fn currency_ref_value(
    state: &TokenBindingState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyId>, TokenBindingStateError> {
    Ok(Some(state.currency_id))
}
