use appletheia::aggregate_state;
use appletheia::domain::{UniqueValue, UniqueValuePart, UniqueValues};
use appletheia::unique_constraints;

use super::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationId,
    OrganizationOwner, OrganizationPictureRef, OrganizationStateError, OrganizationStatus,
    OrganizationWebsiteUrl,
};

/// Stores the materialized state of an `Organization` aggregate.
#[aggregate_state(error = OrganizationStateError)]
#[unique_constraints(entry(key = "handle", values = handle_values))]
pub struct OrganizationState {
    pub(super) id: OrganizationId,
    pub(super) status: OrganizationStatus,
    pub(super) display_name: OrganizationDisplayName,
    pub(super) description: Option<OrganizationDescription>,
    pub(super) website_url: Option<OrganizationWebsiteUrl>,
    pub(super) picture: Option<OrganizationPictureRef>,
    pub(super) handle: OrganizationHandle,
    pub(super) owner: OrganizationOwner,
}

impl OrganizationState {
    /// Creates a new organization state.
    pub(super) fn new(
        id: OrganizationId,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        display_name: OrganizationDisplayName,
        description: Option<OrganizationDescription>,
        website_url: Option<OrganizationWebsiteUrl>,
        picture: Option<OrganizationPictureRef>,
    ) -> Self {
        Self {
            id,
            status: OrganizationStatus::Active,
            display_name,
            description,
            website_url,
            picture,
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
        OrganizationDescription, OrganizationDisplayName, OrganizationPictureRef,
        OrganizationPictureUrl, OrganizationWebsiteUrl,
    };

    use super::{
        OrganizationHandle, OrganizationId, OrganizationOwner, OrganizationState,
        OrganizationStatus,
    };

    fn display_name() -> OrganizationDisplayName {
        OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid")
    }

    #[test]
    fn exposes_id_via_aggregate_state_trait() {
        let id = OrganizationId::new();
        let owner = OrganizationOwner::User(crate::UserId::new());
        let handle = OrganizationHandle::try_from("acme-labs").expect("handle should be valid");
        let state =
            OrganizationState::new(id, owner, handle.clone(), display_name(), None, None, None);

        assert_eq!(state.id(), id);
        assert_eq!(state.handle, handle);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn state_can_store_profile_attributes() {
        let state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name(),
            Some(
                OrganizationDescription::try_from("Independent research lab")
                    .expect("description should be valid"),
            ),
            Some(
                OrganizationWebsiteUrl::try_from("https://acme.example.com")
                    .expect("website URL should be valid"),
            ),
            Some(OrganizationPictureRef::external_url(
                OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
                    .expect("picture URL should be valid"),
            )),
        );

        assert_eq!(state.display_name.value(), "Acme Labs");
        assert!(state.description.is_some());
        assert!(state.website_url.is_some());
        assert!(state.picture.is_some());
    }

    #[test]
    fn active_state_returns_unique_entries_for_handle() {
        let state = OrganizationState::new(
            OrganizationId::new(),
            OrganizationOwner::User(crate::UserId::new()),
            OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name(),
            None,
            None,
            None,
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
            display_name(),
            None,
            None,
            None,
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
