use appletheia::aggregate_state;
use appletheia::domain::{AggregateId, UniqueValue};
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use banking_iam_domain::UserId;
use uuid::Uuid;

use super::{CurrencyRegistrarMembershipStateError, CurrencyRegistrarMembershipStatus};
use crate::currency_registrar::CurrencyRegistrarId;

/// Stores the materialized state of a CurrencyRegistrarMembership aggregate.
#[aggregate_state(error = CurrencyRegistrarMembershipStateError)]
#[unique_constraints(entry(key = "registrar_user", value = registrar_user_unique_value))]
#[reference_indexes(
    entry(key = "currency_registrar", value = currency_registrar_ref_value),
    entry(key = "user", value = user_ref_value)
)]
pub struct CurrencyRegistrarMembershipState {
    pub(super) currency_registrar_id: CurrencyRegistrarId,
    pub(super) user_id: UserId,
    pub(super) status: CurrencyRegistrarMembershipStatus,
}

/// Reserves the registrar/user pair only while this membership is active.
///
/// Removal terminates this aggregate lifecycle and releases the pair so a
/// later membership can use a new aggregate identity.
fn registrar_user_unique_value(
    state: &CurrencyRegistrarMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, CurrencyRegistrarMembershipStateError> {
    if !state.status.is_active() {
        return Ok(None);
    }

    let currency_registrar_id = state.currency_registrar_id.value().to_string();
    let user_id = state.user_id.value().to_string();
    Ok(Some(UniqueValue::from_strings([
        currency_registrar_id.as_str(),
        user_id.as_str(),
    ])?))
}

fn currency_registrar_ref_value(
    state: &CurrencyRegistrarMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<CurrencyRegistrarId>, CurrencyRegistrarMembershipStateError> {
    Ok(Some(state.currency_registrar_id))
}

fn user_ref_value(
    state: &CurrencyRegistrarMembershipState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, CurrencyRegistrarMembershipStateError> {
    Ok(Some(state.user_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{UniqueConstraints, UniqueValues};
    use banking_iam_domain::UserId;
    use uuid::Uuid;

    use super::{CurrencyRegistrarMembershipState, CurrencyRegistrarMembershipStatus};
    use crate::currency_registrar::CurrencyRegistrarId;

    #[test]
    fn active_membership_reserves_the_registrar_user_pair() {
        let state = CurrencyRegistrarMembershipState {
            currency_registrar_id: CurrencyRegistrarId::new(),
            user_id: UserId::new(),
            status: CurrencyRegistrarMembershipStatus::Active,
        };

        let entries = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert_eq!(
            entries
                .get(CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn removed_membership_releases_the_registrar_user_pair() {
        let state = CurrencyRegistrarMembershipState {
            currency_registrar_id: CurrencyRegistrarId::new(),
            user_id: UserId::new(),
            status: CurrencyRegistrarMembershipStatus::Removed,
        };

        let entries = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert_eq!(
            entries.get(CurrencyRegistrarMembershipState::REGISTRAR_USER_KEY),
            None
        );
    }
}
