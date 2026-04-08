use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows owners to edit an organization handle.
pub struct OrganizationHandleEditorRelation;

impl Relation for OrganizationHandleEditorRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("handle_editor"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
