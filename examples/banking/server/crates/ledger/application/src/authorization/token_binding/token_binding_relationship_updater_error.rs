use appletheia::application::authorization::RelationshipStoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenBindingRelationshipUpdaterError {
    #[error("relationship store failed")]
    Store(#[from] RelationshipStoreError),
}
