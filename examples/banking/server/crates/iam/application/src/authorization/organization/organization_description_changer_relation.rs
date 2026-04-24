use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationProfileEditorRelation};

/// Allows organization profile editors to change an organization description.
pub struct OrganizationDescriptionChangerRelation;

impl Relation for OrganizationDescriptionChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("description_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationProfileEditorRelation::REF,
    };
}
