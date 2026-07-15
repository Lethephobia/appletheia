/// Describes why a currency image URL cannot be validated.
#[derive(Debug, thiserror::Error)]
pub enum CurrencyImageUrlError {
    #[error("currency image URL is invalid")]
    Parse(#[from] url::ParseError),
}
