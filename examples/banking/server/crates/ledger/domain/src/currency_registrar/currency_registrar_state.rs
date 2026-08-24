use appletheia::aggregate_state;
use appletheia::domain::UniqueValue;
use appletheia::reference_indexes;
use appletheia::unique_constraints;

use uuid::Uuid;

use super::{
    CurrencyRegistrarDescription, CurrencyRegistrarDisplayName, CurrencyRegistrarHandle,
    CurrencyRegistrarStateError,
};

/// Stores the materialized state of a CurrencyRegistrar aggregate.
#[aggregate_state(error = CurrencyRegistrarStateError)]
#[unique_constraints(entry(key = "handle", value = handle_unique_value))]
#[reference_indexes()]
pub struct CurrencyRegistrarState {
    pub(super) handle: CurrencyRegistrarHandle,
    pub(super) display_name: CurrencyRegistrarDisplayName,
    pub(super) description: Option<CurrencyRegistrarDescription>,
}

fn handle_unique_value(
    state: &CurrencyRegistrarState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, CurrencyRegistrarStateError> {
    Ok(Some(UniqueValue::from_strings([state.handle.as_ref()])?))
}
