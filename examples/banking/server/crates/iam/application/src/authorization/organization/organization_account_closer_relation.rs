use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to close organization-owned accounts.
pub struct OrganizationAccountCloserRelation;

impl Relation for OrganizationAccountCloserRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("account_closer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
