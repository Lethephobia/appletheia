use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::{CurrencyRegistrarJoinRequestStateError, CurrencyRegistrarJoinRequestStatus};

/// Stores the materialized state of an `CurrencyRegistrarJoinRequest` aggregate.
#[aggregate_state(error = CurrencyRegistrarJoinRequestStateError)]
#[unique_constraints(
    entry(key = "registrar_requester", value = registrar_requester_unique_value)
)]
#[reference_indexes(
    entry(key = "registrar", value = registrar_ref_value),
    entry(key = "requester", value = requester_ref_value)
)]
pub struct CurrencyRegistrarJoinRequestState {
    pub(super) currency_registrar_id: CurrencyRegistrarId,
    pub(super) requester_id: UserId,
    pub(super) status: CurrencyRegistrarJoinRequestStatus,
}

fn registrar_requester_unique_value(
    state: &CurrencyRegistrarJoinRequestState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, CurrencyRegistrarJoinRequestStateError> {
    if !state.status.is_pending() {
        return Ok(None);
    }

    let currency_registrar_id = state.currency_registrar_id.value().to_string();
    let requester_id = state.requester_id.value().to_string();
    let value = UniqueValue::from_strings([currency_registrar_id.as_str(), requester_id.as_str()])?;

    Ok(Some(value))
}

fn registrar_ref_value(
    state: &CurrencyRegistrarJoinRequestState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyRegistrarId>, CurrencyRegistrarJoinRequestStateError> {
    Ok(Some(state.currency_registrar_id))
}

fn requester_ref_value(
    state: &CurrencyRegistrarJoinRequestState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, CurrencyRegistrarJoinRequestStateError> {
    Ok(Some(state.requester_id))
}
