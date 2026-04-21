use appletheia::event_payload;

use super::{
    OrganizationEventPayloadError, OrganizationHandle, OrganizationId, OrganizationOwner,
    OrganizationProfile,
};

/// Represents the domain events emitted by an `Organization` aggregate.
#[event_payload(error = OrganizationEventPayloadError)]
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        profile: OrganizationProfile,
    },
    OwnershipTransferred {
        owner: OrganizationOwner,
    },
    HandleChanged {
        handle: OrganizationHandle,
    },
    ProfileChanged {
        profile: OrganizationProfile,
    },
    Removed,
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;
    use crate::OrganizationDisplayName;

    use super::{
        OrganizationEventPayload, OrganizationHandle, OrganizationId, OrganizationOwner,
        OrganizationProfile,
    };

    fn profile() -> OrganizationProfile {
        OrganizationProfile::new(
            OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid"),
            None,
            None,
            None,
        )
    }

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            OrganizationEventPayload::CREATED,
            appletheia::domain::EventName::new("created")
        );
        assert_eq!(
            OrganizationEventPayload::OWNERSHIP_TRANSFERRED,
            appletheia::domain::EventName::new("ownership_transferred")
        );
        assert_eq!(
            OrganizationEventPayload::HANDLE_CHANGED,
            appletheia::domain::EventName::new("handle_changed")
        );
        assert_eq!(
            OrganizationEventPayload::PROFILE_CHANGED,
            appletheia::domain::EventName::new("profile_changed")
        );
        assert_eq!(
            OrganizationEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
    }

    #[test]
    fn profile_changed_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::ProfileChanged { profile: profile() };

        assert_eq!(payload.name(), OrganizationEventPayload::PROFILE_CHANGED);
    }

    #[test]
    fn serializes_profile_changed_payload_to_json() {
        let payload = OrganizationEventPayload::ProfileChanged { profile: profile() };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("profile_changed"));
        assert_eq!(
            value["data"]["profile"]["display_name"],
            serde_json::json!("Acme Labs")
        );
    }

    #[test]
    fn serializes_created_payload_to_json() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            owner: OrganizationOwner::User(crate::UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            profile: profile(),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
        assert_eq!(
            value["data"]["profile"]["display_name"],
            serde_json::json!("Acme Labs")
        );
    }
}
