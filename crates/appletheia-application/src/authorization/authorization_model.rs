use super::{AuthorizationModelError, RelationRefOwned, UsersetExpr};

#[allow(async_fn_in_trait)]
pub trait AuthorizationModel: Send + Sync {
    async fn expr_for(
        &self,
        relation: &RelationRefOwned,
    ) -> Result<Option<UsersetExpr>, AuthorizationModelError>;
}
