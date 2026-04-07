use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::User;

/// Allows directly assigned status managers to manage user status transitions.
pub struct UserStatusManagerRelation;

impl Relation for UserStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("status_manager"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
