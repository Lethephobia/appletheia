use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to change an organization website URL.
pub struct OrganizationWebsiteUrlChangerRelation;

impl Relation for OrganizationWebsiteUrlChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("website_url_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationAdminRelation::REF,
    };
}
