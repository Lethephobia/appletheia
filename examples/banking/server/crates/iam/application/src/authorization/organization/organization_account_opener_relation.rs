use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to open accounts for the organization.
pub struct OrganizationAccountOpenerRelation;

impl Relation for OrganizationAccountOpenerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("account_opener"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationOwnerRelation::REF,
    };
}
