use super::OnchainConfigurerError;

#[allow(async_fn_in_trait)]
pub trait OnchainConfigurer: Send + Sync {
    async fn configure(&self) -> Result<(), OnchainConfigurerError>;
}
