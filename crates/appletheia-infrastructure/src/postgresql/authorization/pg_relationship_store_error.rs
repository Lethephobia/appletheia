use appletheia_application::authorization::{CaveatNameError, RelationNameError};
use appletheia_application::event::AggregateTypeOwnedError;
use appletheia_application::request_context::SubjectKindError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgRelationshipStoreError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("invalid aggregate type: {0}")]
    AggregateType(#[from] AggregateTypeOwnedError),

    #[error("invalid relation name: {0}")]
    RelationName(#[from] RelationNameError),

    #[error("invalid caveat name: {0}")]
    CaveatName(#[from] CaveatNameError),

    #[error("invalid subject kind: {0}")]
    SubjectKind(#[from] SubjectKindError),

    #[error("invalid relationship tuple row")]
    InvalidRow,
}
