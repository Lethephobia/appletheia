use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to invite users.
pub struct OrganizationInviterRelation;

impl Relation for OrganizationInviterRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("inviter"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: OrganizationAdminRelation::REF.into(),
        }
    }
}
