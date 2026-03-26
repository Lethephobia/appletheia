use appletheia::relations;
use banking_iam_domain::Role;

use super::role_assignee_relation::RoleAssigneeRelation;

/// Defines static authorization relations for `Role`.
#[relations(aggregate = Role, relations = [RoleAssigneeRelation])]
pub struct RoleRelations;
