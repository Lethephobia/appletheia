use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation, UserStatusManagerRelation};

/// Allows owners to remove a user.
pub struct UserRemoverRelation;

impl Relation for UserRemoverRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF,
        },
        UsersetExpr::ComputedUserset {
            relation: UserStatusManagerRelation::REF,
        },
    ]);
}
