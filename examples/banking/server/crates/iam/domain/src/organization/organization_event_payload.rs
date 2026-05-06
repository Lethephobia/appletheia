use appletheia::event_payload;

use super::{
    OrganizationDescription, OrganizationDescriptionChangeRejectionReason, OrganizationDisplayName,
    OrganizationDisplayNameChangeRejectionReason, OrganizationEventPayloadError,
    OrganizationHandle, OrganizationHandleChangeRejectionReason, OrganizationId, OrganizationOwner,
    OrganizationOwnershipTransferRejectionReason, OrganizationPictureChangeRejectionReason,
    OrganizationPictureRef, OrganizationRemoveRejectionReason, OrganizationWebsiteUrl,
    OrganizationWebsiteUrlChangeRejectionReason,
};

/// Represents the domain events emitted by an `Organization` aggregate.
#[event_payload(error = OrganizationEventPayloadError)]
pub enum OrganizationEventPayload {
    Created {
        id: OrganizationId,
        owner: OrganizationOwner,
        handle: OrganizationHandle,
        display_name: OrganizationDisplayName,
        description: Option<OrganizationDescription>,
        website_url: Option<OrganizationWebsiteUrl>,
        picture: Option<OrganizationPictureRef>,
    },
    OwnershipTransferred {
        owner: OrganizationOwner,
    },
    OwnershipTransferRejected {
        owner: OrganizationOwner,
        reason: OrganizationOwnershipTransferRejectionReason,
    },
    HandleChanged {
        handle: OrganizationHandle,
    },
    HandleChangeRejected {
        handle: OrganizationHandle,
        reason: OrganizationHandleChangeRejectionReason,
    },
    DisplayNameChanged {
        display_name: OrganizationDisplayName,
    },
    DisplayNameChangeRejected {
        display_name: OrganizationDisplayName,
        reason: OrganizationDisplayNameChangeRejectionReason,
    },
    DescriptionChanged {
        description: Option<OrganizationDescription>,
    },
    DescriptionChangeRejected {
        description: Option<OrganizationDescription>,
        reason: OrganizationDescriptionChangeRejectionReason,
    },
    WebsiteUrlChanged {
        website_url: Option<OrganizationWebsiteUrl>,
    },
    WebsiteUrlChangeRejected {
        website_url: Option<OrganizationWebsiteUrl>,
        reason: OrganizationWebsiteUrlChangeRejectionReason,
    },
    PictureChanged {
        picture: Option<OrganizationPictureRef>,
        old_picture: Option<OrganizationPictureRef>,
    },
    PictureChangeRejected {
        picture: Option<OrganizationPictureRef>,
        reason: OrganizationPictureChangeRejectionReason,
    },
    Removed,
    RemoveRejected {
        reason: OrganizationRemoveRejectionReason,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::{OrganizationDisplayName, OrganizationWebsiteUrl};

    use super::{OrganizationEventPayload, OrganizationHandle, OrganizationId, OrganizationOwner};

    fn display_name() -> OrganizationDisplayName {
        OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid")
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
            OrganizationEventPayload::OWNERSHIP_TRANSFER_REJECTED,
            appletheia::domain::EventName::new("ownership_transfer_rejected")
        );
        assert_eq!(
            OrganizationEventPayload::HANDLE_CHANGED,
            appletheia::domain::EventName::new("handle_changed")
        );
        assert_eq!(
            OrganizationEventPayload::HANDLE_CHANGE_REJECTED,
            appletheia::domain::EventName::new("handle_change_rejected")
        );
        assert_eq!(
            OrganizationEventPayload::DISPLAY_NAME_CHANGED,
            appletheia::domain::EventName::new("display_name_changed")
        );
        assert_eq!(
            OrganizationEventPayload::DISPLAY_NAME_CHANGE_REJECTED,
            appletheia::domain::EventName::new("display_name_change_rejected")
        );
        assert_eq!(
            OrganizationEventPayload::DESCRIPTION_CHANGED,
            appletheia::domain::EventName::new("description_changed")
        );
        assert_eq!(
            OrganizationEventPayload::DESCRIPTION_CHANGE_REJECTED,
            appletheia::domain::EventName::new("description_change_rejected")
        );
        assert_eq!(
            OrganizationEventPayload::WEBSITE_URL_CHANGED,
            appletheia::domain::EventName::new("website_url_changed")
        );
        assert_eq!(
            OrganizationEventPayload::WEBSITE_URL_CHANGE_REJECTED,
            appletheia::domain::EventName::new("website_url_change_rejected")
        );
        assert_eq!(
            OrganizationEventPayload::PICTURE_CHANGED,
            appletheia::domain::EventName::new("picture_changed")
        );
        assert_eq!(
            OrganizationEventPayload::PICTURE_CHANGE_REJECTED,
            appletheia::domain::EventName::new("picture_change_rejected")
        );
        assert_eq!(
            OrganizationEventPayload::REMOVED,
            appletheia::domain::EventName::new("removed")
        );
        assert_eq!(
            OrganizationEventPayload::REMOVE_REJECTED,
            appletheia::domain::EventName::new("remove_rejected")
        );
    }

    #[test]
    fn display_name_changed_payload_name_matches_variant() {
        let payload = OrganizationEventPayload::DisplayNameChanged {
            display_name: display_name(),
        };

        assert_eq!(
            payload.name(),
            OrganizationEventPayload::DISPLAY_NAME_CHANGED
        );
    }

    #[test]
    fn serializes_website_url_changed_payload_to_json() {
        let payload = OrganizationEventPayload::WebsiteUrlChanged {
            website_url: Some(
                OrganizationWebsiteUrl::try_from("https://acme.example.com")
                    .expect("website URL should be valid"),
            ),
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("website_url_changed"));
        assert_eq!(
            value["data"]["website_url"],
            serde_json::json!("https://acme.example.com/")
        );
    }

    #[test]
    fn serializes_created_payload_to_json() {
        let payload = OrganizationEventPayload::Created {
            id: OrganizationId::new(),
            owner: OrganizationOwner::User(crate::UserId::new()),
            handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
            display_name: display_name(),
            description: None,
            website_url: None,
            picture: None,
        };

        let value = payload.into_json_value().expect("payload should serialize");

        assert_eq!(value["type"], serde_json::json!("created"));
        assert_eq!(
            value["data"]["display_name"],
            serde_json::json!("Acme Labs")
        );
    }
}
