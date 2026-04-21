use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Defines the `username_changer` relation for `User`.
pub struct UserUsernameChangerRelation;

impl Relation for UserUsernameChangerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("username_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: UserOwnerRelation::REF,
    };
}
