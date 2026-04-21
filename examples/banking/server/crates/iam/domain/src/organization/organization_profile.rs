use serde::{Deserialize, Serialize};

use super::{
    OrganizationDescription, OrganizationDisplayName, OrganizationPictureUrl,
    OrganizationWebsiteUrl,
};

/// Stores the public profile information for an organization.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OrganizationProfile {
    pub(super) display_name: OrganizationDisplayName,
    pub(super) description: Option<OrganizationDescription>,
    pub(super) website_url: Option<OrganizationWebsiteUrl>,
    pub(super) picture_url: Option<OrganizationPictureUrl>,
}

impl OrganizationProfile {
    /// Creates a new organization profile.
    pub fn new(
        display_name: OrganizationDisplayName,
        description: Option<OrganizationDescription>,
        website_url: Option<OrganizationWebsiteUrl>,
        picture_url: Option<OrganizationPictureUrl>,
    ) -> Self {
        Self {
            display_name,
            description,
            website_url,
            picture_url,
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

    /// Returns the organization picture URL.
    pub fn picture_url(&self) -> Option<&OrganizationPictureUrl> {
        self.picture_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OrganizationDescription, OrganizationDisplayName, OrganizationPictureUrl,
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
            Some(
                OrganizationPictureUrl::try_from("https://cdn.example.com/acme.png")
                    .expect("picture URL should be valid"),
            ),
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
            profile.picture_url().map(|value| value.value().as_str()),
            Some("https://cdn.example.com/acme.png")
        );
    }
}
