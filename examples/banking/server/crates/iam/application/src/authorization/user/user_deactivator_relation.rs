use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Allows owners to deactivate a user.
pub struct UserDeactivatorRelation;

impl Relation for UserDeactivatorRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("deactivator"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF,
        },
    ]);
}
