use appletheia::aggregate_state;
use appletheia::domain::UniqueValue;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use crate::core::{CurrencyCode, CurrencyDecimals};
use crate::currency_registrar::CurrencyRegistrarId;

use super::{CurrencyDescription, CurrencyStateError, CurrencyStatus};

/// Stores the materialized state of a Currency aggregate.
#[aggregate_state(error = CurrencyStateError)]
#[unique_constraints(entry(key = "code", value = currency_code_unique_value))]
#[reference_indexes(entry(key = "currency_registrar", value = currency_registrar_ref_value))]
pub struct CurrencyState {
    pub(super) currency_registrar_id: CurrencyRegistrarId,
    pub(super) code: CurrencyCode,
    pub(super) decimals: CurrencyDecimals,
    pub(super) description: Option<CurrencyDescription>,
    pub(super) status: CurrencyStatus,
}

fn currency_registrar_ref_value(
    state: &CurrencyState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyRegistrarId>, CurrencyStateError> {
    Ok(Some(state.currency_registrar_id))
}

fn currency_code_unique_value(
    state: &CurrencyState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, CurrencyStateError> {
    Ok(Some(UniqueValue::from_strings([state.code.as_ref()])?))
}
