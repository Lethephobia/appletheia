use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::User;

/// Allows the owning user itself.
pub struct UserOwnerRelation;

impl Relation for UserOwnerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
