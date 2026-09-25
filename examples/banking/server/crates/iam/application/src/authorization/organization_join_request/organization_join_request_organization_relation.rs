use super::OrganizationJoinRequestOrganizationDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationJoinRequest};

/// Links a join request to its organization.
pub struct OrganizationJoinRequestOrganizationRelation;

impl Relation for OrganizationJoinRequestOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationJoinRequest::TYPE,
        RelationName::new("organization"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationJoinRequest,
            _,
            OrganizationJoinRequestOrganizationDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationJoinRequest, Organization>(
                aggregate.aggregate_id(),
                *aggregate.organization_id()?,
            );
            Ok(entries)
        })
    }
}
