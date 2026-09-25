use appletheia_domain::Aggregate;

use super::{Relationship, RelationshipDeriverError};

/// Derives the current relationships contributed by an aggregate during save.
pub trait RelationshipDeriver: Send + Sync {
    fn derive<A: Aggregate>(
        &self,
        aggregate: &A,
    ) -> Result<Vec<Relationship>, RelationshipDeriverError>;
}
