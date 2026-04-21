use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{
    OrganizationHandle, OrganizationId, OrganizationOwner, OrganizationProfile,
    OrganizationStateError, OrganizationStatus,
};

/// Stores the materialized state of an `Organization` aggregate.
#[aggregate_state(error = OrganizationStateError)]
#[unique_constraints(entry(key = "handle", values = handle_values))]
pub struct OrganizationState {
    pub(super) id: OrganizationId,
    pub(super) status: OrganizationStatus,
    pub(super) profile: OrganizationProfile,
    pub(super) handle: OrganizationHandle,
    pub(super) owner: OrganizationOwner,
}

impl OrganizationState {
    /// Creates a new organization state.
    pub(super) fn new(
        id: OrganizationId,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        profile: OrganizationProfile,
    ) -> Self {
        Self {
            id,
            status: OrganizationStatus::Active,
            profile,
            handle,
            owner,
        }
    }
}

fn handle_values(
    state: &OrganizationState,
) -> Result<Option<UniqueValues>, OrganizationStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let part = UniqueValuePart::try_from(state.handle.as_ref())?;
    let value = UniqueValue::new(vec![part])?;
    let values = UniqueValues::new(vec![value])?;

    Ok(Some(values))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{AggregateState, UniqueConstraints, UniqueValues};

    use crate::{
        OrganizationDescription, OrganizationDisplayName, OrganizationPictureUrl,
        OrganizationWebsiteUrl,
    };

    use super::{
        OrganizationHandle, OrganizationId, OrganizationOwner, OrganizationProfile,
        OrganizationState, OrganizationStatus,
    };

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationId::new();
        let owner = OrganizationOwner::User(crate::UserId::new());
        let handle = OrganizationHandle::try_from("acme-labs").expect("handle should be valid");
        let state = OrganizationState::new(
            id,
            owner,
            handle.clone(),
            OrganizationProfile::new(
                OrganizationDisplayName::try_from("Acme Labs")
                    .expect("display name should be valid"),
                None,
                None,
                None,
            ),
        );

        assert_eq!(state.id(), id);
        assert_eq!(state.handle, handle);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn state_can_store_profile() {
        let state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            OrganizationProfile::new(
                OrganizationDisplayName::try_from("Acme Labs")
                    .expect("display name should be valid"),
                Some(
                    OrganizationDescription::try_from("Independent research lab")
                        .expect("description should be valid"),
                ),
                Some(
                    OrganizationWebsiteUrl::try_from("https://acme.example.com")
                        .expect("website URL should be valid"),
                ),
                Some(
                    OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
                        .expect("picture URL should be valid"),
                ),
            ),
        );

        assert_eq!(state.profile.display_name().value(), "Acme Labs");
    }

    #[test]
    fn active_state_returns_unique_entries_for_handle() {
        let state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            OrganizationProfile::new(
                OrganizationDisplayName::try_from("Acme Labs")
                    .expect("display name should be valid"),
                None,
                None,
                None,
            ),
        );

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::HANDLE_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn removed_state_has_no_handle_unique_entry() {
        let mut state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            OrganizationProfile::new(
                OrganizationDisplayName::try_from("Acme Labs")
                    .expect("display name should be valid"),
                None,
                None,
                None,
            ),
        );
        state.status = OrganizationStatus::Removed;

        let entries = state.unique_entries().expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::HANDLE_KEY)
                .map(UniqueValues::len),
            None
        );
    }
}
