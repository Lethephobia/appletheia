use banking_iam_domain::{UserPictureObjectName, UserPictureRef, UserPictureUrl};

use super::pg_user_picture_ref_columns_error::PgUserPictureRefColumnsError;

/// PostgreSQL columns that encode a user picture reference.
#[derive(Debug)]
pub(crate) struct PgUserPictureRefColumns {
    pub picture_type: Option<String>,
    pub object_name: Option<String>,
    pub external_url: Option<String>,
}

impl PgUserPictureRefColumns {
    const EXTERNAL_URL: &'static str = "external_url";
    const OBJECT_NAME: &'static str = "object_name";

    pub(crate) fn into_picture(
        self,
    ) -> Result<Option<UserPictureRef>, PgUserPictureRefColumnsError> {
        match (
            self.picture_type.as_deref(),
            self.object_name,
            self.external_url,
        ) {
            (None, None, None) => Ok(None),
            (Some(Self::OBJECT_NAME), Some(object_name), None) => {
                let object_name = UserPictureObjectName::try_from(object_name)
                    .map_err(|error| PgUserPictureRefColumnsError::ObjectName(Box::new(error)))?;
                Ok(Some(UserPictureRef::object_name(object_name)))
            }
            (Some(Self::EXTERNAL_URL), None, Some(url)) => {
                let url = UserPictureUrl::try_from(url)
                    .map_err(|error| PgUserPictureRefColumnsError::ExternalUrl(Box::new(error)))?;
                Ok(Some(UserPictureRef::external_url(url)))
            }
            (Some(value), _, _) if value != Self::OBJECT_NAME && value != Self::EXTERNAL_URL => {
                Err(PgUserPictureRefColumnsError::UnknownType(value.to_owned()))
            }
            _ => Err(PgUserPictureRefColumnsError::InconsistentColumns),
        }
    }
}
