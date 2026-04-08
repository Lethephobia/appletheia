use std::collections::HashMap;
use std::sync::Arc;

use super::{
    AuthorizationModel, AuthorizationModelError, Relation, RelationRefOwned, UsersetExprOwned,
};

#[derive(Clone, Debug, Default)]
pub struct InMemoryAuthorizationModel {
    exprs: HashMap<RelationRefOwned, Arc<UsersetExprOwned>>,
}

impl InMemoryAuthorizationModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_expr(&mut self, relation: RelationRefOwned, expr: UsersetExprOwned) {
        self.exprs.insert(relation, Arc::new(expr));
    }

    pub fn define_relation<R>(&mut self)
    where
        R: Relation,
    {
        self.define_expr(
            RelationRefOwned::from(R::REF),
            UsersetExprOwned::from(&R::EXPR),
        );
    }
}

impl AuthorizationModel for InMemoryAuthorizationModel {
    async fn expr_for(
        &self,
        relation: &RelationRefOwned,
    ) -> Result<Option<UsersetExprOwned>, AuthorizationModelError> {
        Ok(self.exprs.get(relation).map(|expr| (**expr).clone()))
    }
}
