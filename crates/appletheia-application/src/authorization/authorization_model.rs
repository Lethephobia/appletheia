use super::{AuthorizationModelError, RelationRefOwned, UsersetExprOwned};

#[allow(async_fn_in_trait)]
pub trait AuthorizationModel: Send + Sync {
    async fn expr_for(
        &self,
        relation: &RelationRefOwned,
    ) -> Result<Option<UsersetExprOwned>, AuthorizationModelError>;
}
