use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Defines the `username_setter` relation for `User`.
pub struct UserUsernameSetterRelation;

impl Relation for UserUsernameSetterRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("username_setter"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF.into(),
        }
    }
}
