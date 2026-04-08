use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Defines the `profile_editor` relation for `User`.
pub struct UserProfileEditorRelation;

impl Relation for UserProfileEditorRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("profile_editor"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: UserOwnerRelation::REF,
        },
    ]);
}
