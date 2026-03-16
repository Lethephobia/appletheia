use std::collections::HashMap;

use super::{RelationNameOwned, UsersetExpr};

#[derive(Clone, Debug, Default)]
pub struct AuthorizationTypeDefinition {
    relations: HashMap<RelationNameOwned, UsersetExpr>,
}

impl AuthorizationTypeDefinition {
    pub fn new(relations: HashMap<RelationNameOwned, UsersetExpr>) -> Self {
        Self { relations }
    }

    pub fn define_relation<R>(&mut self, relation: R, expr: UsersetExpr)
    where
        R: Into<RelationNameOwned>,
    {
        self.relations.insert(relation.into(), expr);
    }

    pub fn is_defined(&self, relation: &RelationNameOwned) -> bool {
        self.relations.contains_key(relation)
    }

    pub fn expr_for(&self, relation: &RelationNameOwned) -> Option<&UsersetExpr> {
        self.relations.get(relation)
    }
}
