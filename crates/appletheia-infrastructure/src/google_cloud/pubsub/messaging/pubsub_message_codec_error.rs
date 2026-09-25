use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PubsubMessageCodecError {
    #[error("encode error")]
    Encode(#[source] Box<dyn Error + Send + Sync>),

    #[error("decode error")]
    Decode(#[source] Box<dyn Error + Send + Sync>),

    #[error("encode selector error")]
    EncodeSelector(#[source] Box<dyn Error + Send + Sync>),
}
