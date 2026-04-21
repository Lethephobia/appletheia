use serde::{Deserialize, Serialize};

use super::{
    OrganizationDescription, OrganizationDisplayName, OrganizationPictureRef,
    OrganizationWebsiteUrl,
};

/// Stores the public profile information for an organization.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OrganizationProfile {
    pub(super) display_name: OrganizationDisplayName,
    pub(super) description: Option<OrganizationDescription>,
    pub(super) website_url: Option<OrganizationWebsiteUrl>,
    pub(super) picture: Option<OrganizationPictureRef>,
}

impl OrganizationProfile {
    /// Creates a new organization profile.
    pub fn new(
        display_name: OrganizationDisplayName,
        description: Option<OrganizationDescription>,
        website_url: Option<OrganizationWebsiteUrl>,
        picture: Option<OrganizationPictureRef>,
    ) -> Self {
        Self {
            display_name,
            description,
            website_url,
            picture,
        }
    }

    /// Returns the organization display name.
    pub fn display_name(&self) -> &OrganizationDisplayName {
        &self.display_name
    }

    /// Returns the organization description.
    pub fn description(&self) -> Option<&OrganizationDescription> {
        self.description.as_ref()
    }

    /// Returns the organization website URL.
    pub fn website_url(&self) -> Option<&OrganizationWebsiteUrl> {
        self.website_url.as_ref()
    }

    /// Returns the organization picture.
    pub fn picture(&self) -> Option<&OrganizationPictureRef> {
        self.picture.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::OrganizationPictureObjectName;

    use super::{
        OrganizationDescription, OrganizationDisplayName, OrganizationPictureRef,
        OrganizationProfile, OrganizationWebsiteUrl,
    };

    #[test]
    fn exposes_stored_values() {
        let profile = OrganizationProfile::new(
            OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid"),
            Some(
                OrganizationDescription::try_from("Independent research lab")
                    .expect("description should be valid"),
            ),
            Some(
                OrganizationWebsiteUrl::try_from("https://acme.example.com")
                    .expect("website URL should be valid"),
            ),
            Some(OrganizationPictureRef::object_name(
                OrganizationPictureObjectName::try_from(
                    "organizations/00000000-0000-0000-0000-000000000001/picture",
                )
                .expect("picture object name should be valid"),
            )),
        );

        assert_eq!(profile.display_name().value(), "Acme Labs");
        assert_eq!(
            profile.description().map(OrganizationDescription::value),
            Some("Independent research lab")
        );
        assert_eq!(
            profile.website_url().map(|value| value.value().as_str()),
            Some("https://acme.example.com/")
        );
        assert_eq!(
            profile
                .picture()
                .and_then(OrganizationPictureRef::as_object_name)
                .map(OrganizationPictureObjectName::value),
            Some("organizations/00000000-0000-0000-0000-000000000001/picture")
        );
    }
}
