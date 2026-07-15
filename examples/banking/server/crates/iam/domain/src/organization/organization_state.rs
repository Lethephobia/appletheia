use appletheia::aggregate_state;
use appletheia::domain::UniqueValue;
use appletheia::reference_indexes;
use appletheia::unique_constraints;
use uuid::Uuid;

use super::{
    OrganizationDescription, OrganizationDisplayName, OrganizationHandle, OrganizationOwner,
    OrganizationPictureRef, OrganizationStateError, OrganizationStatus, OrganizationWebsiteUrl,
};
use crate::UserId;

/// Stores the materialized state of an `Organization` aggregate.
#[aggregate_state(error = OrganizationStateError)]
#[unique_constraints(entry(key = "handle", value = handle_unique_value))]
#[reference_indexes(entry(key = "owner_user", value = owner_user_ref_value))]
pub struct OrganizationState {
    pub(super) owner: OrganizationOwner,
    pub(super) handle: OrganizationHandle,
    pub(super) display_name: OrganizationDisplayName,
    pub(super) description: Option<OrganizationDescription>,
    pub(super) website_url: Option<OrganizationWebsiteUrl>,
    pub(super) picture: Option<OrganizationPictureRef>,
    pub(super) status: OrganizationStatus,
}

fn handle_unique_value(
    state: &OrganizationState,
    _aggregate_id: Uuid,
) -> Result<Option<UniqueValue>, OrganizationStateError> {
    if state.status.is_removed() {
        return Ok(None);
    }

    let value = UniqueValue::from_strings([state.handle.as_ref()])?;

    Ok(Some(value))
}

fn owner_user_ref_value(
    state: &OrganizationState,
    _aggregate_id: Uuid,
) -> Result<Option<UserId>, OrganizationStateError> {
    let OrganizationOwner::User(user_id) = state.owner;

    Ok(Some(user_id))
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{ReferenceIndexes, ReferenceValues, UniqueConstraints, UniqueValues};
    use uuid::Uuid;

    use crate::{
        OrganizationDescription, OrganizationDisplayName, OrganizationPictureRef,
        OrganizationPictureUrl, OrganizationWebsiteUrl, UserId,
    };

    use super::{OrganizationHandle, OrganizationOwner, OrganizationState, OrganizationStatus};

    fn display_name() -> OrganizationDisplayName {
        OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid")
    }

    #[test]
    fn state_stores_domain_attributes() {
        let owner = OrganizationOwner::User(UserId::new());
        let handle = OrganizationHandle::try_from("acme-labs").expect("handle should be valid");
        let state = OrganizationState {
            owner,
            handle: handle.clone(),
            display_name: display_name(),
            description: None,
            website_url: None,
            picture: None,
            status: OrganizationStatus::Active,
        };
        assert_eq!(state.handle, handle);
        assert_eq!(state.owner, owner);
    }

    #[test]
    fn state_can_store_profile_attributes() {
        let state = OrganizationState {
            owner: OrganizationOwner::User(UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name: display_name(),
            description: Some(
                OrganizationDescription::try_from("Independent research lab")
                    .expect("description should be valid"),
            ),
            website_url: Some(
                OrganizationWebsiteUrl::try_from("https://acme.example.com")
                    .expect("website URL should be valid"),
            ),
            picture: Some(OrganizationPictureRef::external_url(
                OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
                    .expect("picture URL should be valid"),
            )),
            status: OrganizationStatus::Active,
        };

        assert_eq!(state.display_name.value(), "Acme Labs");
        assert!(state.description.is_some());
        assert!(state.website_url.is_some());
        assert!(state.picture.is_some());
    }

    #[test]
    fn active_state_returns_unique_entries_for_handle() {
        let state = OrganizationState {
            owner: OrganizationOwner::User(UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name: display_name(),
            description: None,
            website_url: None,
            picture: None,
            status: OrganizationStatus::Active,
        };

        let entries = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::HANDLE_KEY)
                .map(UniqueValues::len),
            Some(1)
        );
    }

    #[test]
    fn removed_state_has_no_handle_unique_entry() {
        let mut state = OrganizationState {
            owner: OrganizationOwner::User(UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name: display_name(),
            description: None,
            website_url: None,
            picture: None,
            status: OrganizationStatus::Active,
        };
        state.status = OrganizationStatus::Removed;

        let entries = state
            .unique_entries(Uuid::now_v7())
            .expect("unique entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::HANDLE_KEY)
                .map(UniqueValues::len),
            None
        );
    }

    #[test]
    fn returns_reference_entry_for_owner_user() {
        let owner = OrganizationOwner::User(UserId::new());
        let state = OrganizationState {
            owner,
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name: display_name(),
            description: None,
            website_url: None,
            picture: None,
            status: OrganizationStatus::Active,
        };

        let entries = state
            .reference_entries(Uuid::now_v7())
            .expect("reference entries should build");

        assert_eq!(
            entries
                .get(OrganizationState::OWNER_USER_REF)
                .map(ReferenceValues::len),
            Some(1)
        );
    }
}
