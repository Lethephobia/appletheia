use super::CurrencyRegistrarJoinRequestRequesterDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::User;
use banking_ledger_domain::CurrencyRegistrarJoinRequest;

/// Links a join request to the user who submitted membership.
pub struct CurrencyRegistrarJoinRequestRequesterRelation;

impl Relation for CurrencyRegistrarJoinRequestRequesterRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarJoinRequest::TYPE,
        RelationName::new("requester"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            CurrencyRegistrarJoinRequest,
            _,
            CurrencyRegistrarJoinRequestRequesterDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<CurrencyRegistrarJoinRequest, User>(
                aggregate.aggregate_id(),
                *aggregate.requester_id()?,
            );
            Ok(entries)
        })
    }
}
