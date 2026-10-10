use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationJoinRequest, OrganizationJoinRequestOrganizationRelation};
use crate::OrganizationOwnerRelation;

/// Allows organization owners to manage join requests.
pub struct OrganizationJoinRequestManagerRelation;

impl Relation for OrganizationJoinRequestManagerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationJoinRequest::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationJoinRequestOrganizationRelation::REF.into(),
            computed_userset: OrganizationOwnerRelation::REF.into(),
        }
    }
}
