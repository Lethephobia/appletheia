use thiserror::Error;

use super::{
    ReadModelAttributeKeyError, ReadModelAttributeValueError, ReadModelRelationshipError,
    ReadModelRelationshipKeyError, ReadModelResourceKey,
};

#[derive(Debug, Error)]
pub enum ReadModelError {
    #[error(transparent)]
    AttributeKey(#[from] ReadModelAttributeKeyError),

    #[error(transparent)]
    AttributeValue(#[from] ReadModelAttributeValueError),

    #[error(transparent)]
    RelationshipKey(#[from] ReadModelRelationshipKeyError),

    #[error(transparent)]
    Relationship(#[from] ReadModelRelationshipError),

    #[error("duplicate resource field: {0}")]
    DuplicateField(ReadModelResourceKey),
}
