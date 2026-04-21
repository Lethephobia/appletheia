use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use appletheia::domain::AggregateId;

use super::{OrganizationId, OrganizationPictureObjectNameError};

/// Represents an organization picture object name in object storage.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrganizationPictureObjectName(String);

impl OrganizationPictureObjectName {
    /// Creates a new picture object name for the given organization.
    pub fn new(organization_id: OrganizationId) -> Self {
        Self(format!("organizations/{}/picture", organization_id.value()))
    }

    /// Returns the picture object name.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OrganizationPictureObjectName {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

impl Display for OrganizationPictureObjectName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl FromStr for OrganizationPictureObjectName {
    type Err = OrganizationPictureObjectNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(OrganizationPictureObjectNameError::Empty);
        }

        let segments = value.split('/').collect::<Vec<_>>();
        if segments.len() != 3 || segments[0] != "organizations" || segments[2] != "picture" {
            return Err(OrganizationPictureObjectNameError::InvalidFormat);
        }

        OrganizationId::try_from_uuid(
            Uuid::parse_str(segments[1])
                .map_err(|_| OrganizationPictureObjectNameError::InvalidFormat)?,
        )
        .map_err(|_| OrganizationPictureObjectNameError::InvalidFormat)?;

        Ok(Self(value.to_owned()))
    }
}

impl TryFrom<&str> for OrganizationPictureObjectName {
    type Error = OrganizationPictureObjectNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for OrganizationPictureObjectName {
    type Error = OrganizationPictureObjectNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<OrganizationPictureObjectName> for String {
    fn from(value: OrganizationPictureObjectName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use appletheia::domain::AggregateId;

    use super::{
        OrganizationId, OrganizationPictureObjectName, OrganizationPictureObjectNameError,
    };

    #[test]
    fn new_generates_picture_object_name_for_organization() {
        let organization_id =
            OrganizationId::try_from_uuid(Uuid::nil()).expect("organization ID should be valid");
        let object_name = OrganizationPictureObjectName::new(organization_id);

        assert_eq!(
            object_name.value(),
            "organizations/00000000-0000-0000-0000-000000000000/picture"
        );
    }

    #[test]
    fn try_from_accepts_valid_picture_object_name() {
        let object_name = OrganizationPictureObjectName::try_from(
            "organizations/00000000-0000-0000-0000-000000000001/picture",
        )
        .expect("name should be valid");

        assert_eq!(
            object_name.value(),
            "organizations/00000000-0000-0000-0000-000000000001/picture"
        );
    }

    #[test]
    fn try_from_rejects_invalid_picture_object_name() {
        let error = OrganizationPictureObjectName::try_from("organizations/not-a-uuid/picture")
            .expect_err("name should be invalid");

        assert!(matches!(
            error,
            OrganizationPictureObjectNameError::InvalidFormat
        ));
    }
}
