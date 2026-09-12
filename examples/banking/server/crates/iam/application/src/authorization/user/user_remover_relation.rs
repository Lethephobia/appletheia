use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Allows owners to remove a user.
pub struct UserRemoverRelation;

impl Relation for UserRemoverRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("remover"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF.into(),
        }
    }
}
