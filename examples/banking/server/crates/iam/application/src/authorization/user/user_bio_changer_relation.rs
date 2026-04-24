use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{User, UserProfileEditorRelation};

/// Defines the `bio_changer` relation for `User`.
pub struct UserBioChangerRelation;

impl Relation for UserBioChangerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("bio_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: UserProfileEditorRelation::REF,
    };
}
