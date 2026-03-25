use appletheia::application::authorization::{AuthorizationTypeDefinition, Relations};
use appletheia::domain::{Aggregate, AggregateType};
use banking_iam_domain::Role;

use super::role_assignee_relation::RoleAssigneeRelation;

/// Defines static authorization relations for `Role`.
pub struct RoleRelations;

impl Relations for RoleRelations {
    const AGGREGATE_TYPE: AggregateType = Role::TYPE;

    fn build(&self) -> AuthorizationTypeDefinition {
        let mut definition = AuthorizationTypeDefinition::default();
        definition.define_static_relation(RoleAssigneeRelation);
        definition
    }
}
