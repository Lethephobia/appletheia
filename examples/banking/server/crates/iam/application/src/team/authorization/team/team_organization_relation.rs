use super::TeamOrganizationDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, Team};

/// Links active teams to their organization so deletion revokes inherited team access.
pub struct TeamOrganizationRelation;

impl Relation for TeamOrganizationRelation {
    const REF: RelationRef = RelationRef::new(Team::TYPE, RelationName::new("organization"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<Team, _, TeamOrganizationDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            if aggregate.is_active()? {
                entries.insert::<Team, Organization>(
                    aggregate.aggregate_id(),
                    aggregate.organization_id()?,
                );
            }
            Ok(entries)
        })
    }
}
