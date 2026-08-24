use appletheia::application::authorization::RelationshipStoreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyRegistrarMembershipRelationshipUpdaterError {
    #[error("relationship store failed")]
    RelationshipStore(#[from] RelationshipStoreError),
}
