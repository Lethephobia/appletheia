use serde::{Deserialize, Serialize};

use super::{OrganizationPictureObjectName, OrganizationPictureUrl};

/// Represents an organization picture reference stored by the domain.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum OrganizationPictureRef {
    ObjectName(OrganizationPictureObjectName),
    ExternalUrl(OrganizationPictureUrl),
}

impl OrganizationPictureRef {
    /// Creates an organization picture reference backed by object storage.
    pub fn object_name(object_name: OrganizationPictureObjectName) -> Self {
        Self::ObjectName(object_name)
    }

    /// Creates an organization picture reference backed by an external URL.
    pub fn external_url(url: OrganizationPictureUrl) -> Self {
        Self::ExternalUrl(url)
    }

    /// Returns the object name when this picture is stored in object storage.
    pub fn as_object_name(&self) -> Option<&OrganizationPictureObjectName> {
        match self {
            Self::ObjectName(value) => Some(value),
            Self::ExternalUrl(_) => None,
        }
    }

    /// Returns the external URL when this picture is hosted outside object storage.
    pub fn as_external_url(&self) -> Option<&OrganizationPictureUrl> {
        match self {
            Self::ObjectName(_) => None,
            Self::ExternalUrl(value) => Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OrganizationPictureObjectName, OrganizationPictureRef, OrganizationPictureUrl};

    #[test]
    fn returns_object_name_when_present() {
        let picture = OrganizationPictureRef::object_name(
            OrganizationPictureObjectName::try_from(
                "organizations/00000000-0000-0000-0000-000000000001/pictures/00000000-0000-0000-0000-000000000002",
            )
            .expect("name should be valid"),
        );

        assert_eq!(
            picture
                .as_object_name()
                .map(OrganizationPictureObjectName::value),
            Some(
                "organizations/00000000-0000-0000-0000-000000000001/pictures/00000000-0000-0000-0000-000000000002"
            )
        );
        assert_eq!(picture.as_external_url(), None);
    }

    #[test]
    fn returns_external_url_when_present() {
        let picture = OrganizationPictureRef::external_url(
            OrganizationPictureUrl::try_from("https://cdn.example.com/pictures/acme.png")
                .expect("URL should be valid"),
        );

        assert_eq!(
            picture
                .as_external_url()
                .map(|value| value.value().as_str()),
            Some("https://cdn.example.com/pictures/acme.png")
        );
        assert_eq!(picture.as_object_name(), None);
    }
}
