use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

use super::{
    CurrencyRegistrarInvitationExpiresAt, CurrencyRegistrarInvitationIssuer,
    CurrencyRegistrarInvitationStateError, CurrencyRegistrarInvitationStatus,
};

/// Stores the materialized state of an `CurrencyRegistrarInvitation` aggregate.
#[aggregate_state(error = CurrencyRegistrarInvitationStateError)]
#[unique_constraints(
    entry(key = "registrar_invitee", value = registrar_invitee_unique_value)
)]
#[reference_indexes(
    entry(key = "registrar", value = registrar_ref_value),
    entry(key = "invitee", value = invitee_ref_value),
    entry(key = "issuer_user", value = issuer_user_ref_value)
)]
pub struct CurrencyRegistrarInvitationState {
    pub(super) currency_registrar_id: CurrencyRegistrarId,
    pub(super) invitee_id: UserId,
    pub(super) issuer: CurrencyRegistrarInvitationIssuer,
    pub(super) expires_at: CurrencyRegistrarInvitationExpiresAt,
    pub(super) status: CurrencyRegistrarInvitationStatus,
}

fn registrar_invitee_unique_value(
    state: &CurrencyRegistrarInvitationState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, CurrencyRegistrarInvitationStateError> {
    if !state.status.is_pending() {
        return Ok(None);
    }

    let currency_registrar_id = state.currency_registrar_id.value().to_string();
    let invitee_id = state.invitee_id.value().to_string();
    let value = UniqueValue::from_strings([currency_registrar_id.as_str(), invitee_id.as_str()])?;

    Ok(Some(value))
}

fn registrar_ref_value(
    state: &CurrencyRegistrarInvitationState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyRegistrarId>, CurrencyRegistrarInvitationStateError> {
    Ok(Some(state.currency_registrar_id))
}

fn invitee_ref_value(
    state: &CurrencyRegistrarInvitationState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, CurrencyRegistrarInvitationStateError> {
    Ok(Some(state.invitee_id))
}

fn issuer_user_ref_value(
    state: &CurrencyRegistrarInvitationState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, CurrencyRegistrarInvitationStateError> {
    let user_id = match state.issuer {
        CurrencyRegistrarInvitationIssuer::User(user_id) => Some(user_id),
        CurrencyRegistrarInvitationIssuer::System => None,
    };

    Ok(user_id)
}
