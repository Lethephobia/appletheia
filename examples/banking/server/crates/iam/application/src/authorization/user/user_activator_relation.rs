use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Allows owners to activate a user.
pub struct UserActivatorRelation;

impl Relation for UserActivatorRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("activator"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF.into(),
        }
    }
}
