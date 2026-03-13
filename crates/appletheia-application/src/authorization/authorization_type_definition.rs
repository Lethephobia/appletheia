use std::collections::HashMap;

use super::{RelationName, RelationNameOwned, UsersetExpr};

#[derive(Clone, Debug, Default)]
pub struct AuthorizationTypeDefinition {
    relations: HashMap<RelationName, UsersetExpr>,
}

impl AuthorizationTypeDefinition {
    pub fn new(relations: HashMap<RelationName, UsersetExpr>) -> Self {
        Self { relations }
    }

    pub fn define_relation(&mut self, relation: RelationName, expr: UsersetExpr) {
        self.relations.insert(relation, expr);
    }

    pub fn is_defined(&self, relation: &RelationNameOwned) -> bool {
        self.relations.contains_key(relation.value())
    }

    pub fn expr_for(&self, relation: &RelationNameOwned) -> Option<&UsersetExpr> {
        self.relations.get(relation.value())
    }
}
