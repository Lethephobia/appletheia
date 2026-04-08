use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to rename organization-owned accounts.
pub struct OrganizationAccountRenamerRelation;

impl Relation for OrganizationAccountRenamerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("account_renamer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
