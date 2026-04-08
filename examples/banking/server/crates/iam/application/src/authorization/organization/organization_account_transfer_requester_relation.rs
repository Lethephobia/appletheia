use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows organization owners to request transfers from organization-owned accounts.
pub struct OrganizationAccountTransferRequesterRelation;

impl Relation for OrganizationAccountTransferRequesterRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("account_transfer_requester"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
