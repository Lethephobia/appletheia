use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserOwnerRelation};

/// Defines the `picture_changer` relation for `User`.
pub struct UserPictureChangerRelation;

impl Relation for UserPictureChangerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("picture_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: UserOwnerRelation::REF,
    };
}
