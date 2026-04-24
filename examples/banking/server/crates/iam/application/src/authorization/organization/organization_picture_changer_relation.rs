use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationProfileEditorRelation};

/// Allows organization profile editors to change an organization picture.
pub struct OrganizationPictureChangerRelation;

impl Relation for OrganizationPictureChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("picture_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationProfileEditorRelation::REF,
    };
}
