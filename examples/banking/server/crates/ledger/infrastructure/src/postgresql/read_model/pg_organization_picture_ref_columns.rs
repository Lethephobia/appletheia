use banking_iam_domain::{
    OrganizationPictureObjectName, OrganizationPictureRef, OrganizationPictureUrl,
};

use super::pg_organization_picture_ref_columns_error::PgOrganizationPictureRefColumnsError;

/// PostgreSQL columns that encode an organization picture reference.
#[derive(Debug)]
pub(crate) struct PgOrganizationPictureRefColumns {
    pub picture_type: Option<String>,
    pub object_name: Option<String>,
    pub external_url: Option<String>,
}

impl PgOrganizationPictureRefColumns {
    const EXTERNAL_URL: &'static str = "external_url";
    const OBJECT_NAME: &'static str = "object_name";

    pub(crate) fn into_picture(
        self,
    ) -> Result<Option<OrganizationPictureRef>, PgOrganizationPictureRefColumnsError> {
        match (
            self.picture_type.as_deref(),
            self.object_name,
            self.external_url,
        ) {
            (None, None, None) => Ok(None),
            (Some(Self::OBJECT_NAME), Some(object_name), None) => {
                let object_name =
                    OrganizationPictureObjectName::try_from(object_name).map_err(|error| {
                        PgOrganizationPictureRefColumnsError::ObjectName(Box::new(error))
                    })?;
                Ok(Some(OrganizationPictureRef::object_name(object_name)))
            }
            (Some(Self::EXTERNAL_URL), None, Some(url)) => {
                let url = OrganizationPictureUrl::try_from(url).map_err(|error| {
                    PgOrganizationPictureRefColumnsError::ExternalUrl(Box::new(error))
                })?;
                Ok(Some(OrganizationPictureRef::external_url(url)))
            }
            (Some(value), _, _) if value != Self::OBJECT_NAME && value != Self::EXTERNAL_URL => {
                Err(PgOrganizationPictureRefColumnsError::UnknownType(
                    value.to_owned(),
                ))
            }
            _ => Err(PgOrganizationPictureRefColumnsError::InconsistentColumns),
        }
    }
}
