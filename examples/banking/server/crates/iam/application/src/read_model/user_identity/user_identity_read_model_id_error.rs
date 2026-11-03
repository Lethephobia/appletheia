#[derive(Debug, thiserror::Error)]
pub enum UserIdentityReadModelIdError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
