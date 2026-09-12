use super::OrganizationJoinRequestRequesterDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationJoinRequest, User};

/// Links a join request to the user who submitted membership.
pub struct OrganizationJoinRequestRequesterRelation;

impl Relation for OrganizationJoinRequestRequesterRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationJoinRequest::TYPE,
        RelationName::new("requester"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationJoinRequest,
            _,
            OrganizationJoinRequestRequesterDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationJoinRequest, User>(
                aggregate.aggregate_id(),
                *aggregate.requester_id()?,
            );
            Ok(entries)
        })
    }
}
