use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationJoinRequestOrganizationRelation;
use crate::organization::authorization::OrganizationAdminRelation;
use banking_iam_domain::OrganizationJoinRequest;

/// Allows organization admins to manage join requests.
pub struct OrganizationJoinRequestManagerRelation;

impl Relation for OrganizationJoinRequestManagerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationJoinRequest::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationJoinRequestOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
