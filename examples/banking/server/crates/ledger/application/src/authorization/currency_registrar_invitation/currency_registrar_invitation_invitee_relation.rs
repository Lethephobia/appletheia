use super::CurrencyRegistrarInvitationInviteeDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::User;
use banking_ledger_domain::CurrencyRegistrarInvitation;

/// Allows the invited user to act on the invitation.
pub struct CurrencyRegistrarInvitationInviteeRelation;

impl Relation for CurrencyRegistrarInvitationInviteeRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarInvitation::TYPE,
        RelationName::new("invitee"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            CurrencyRegistrarInvitation,
            _,
            CurrencyRegistrarInvitationInviteeDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<CurrencyRegistrarInvitation, User>(
                aggregate.aggregate_id(),
                *aggregate.invitee_id()?,
            );
            Ok(entries)
        })
    }
}
