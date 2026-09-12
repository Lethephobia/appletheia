use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Allows owners to edit user profile attributes.
pub struct UserProfileEditorRelation;

impl Relation for UserProfileEditorRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("profile_editor"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF.into(),
        }
    }
}
