#[derive(Copy, Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum ReadModelWindowSizeError {
    #[error("window size must be greater than zero")]
    Zero,
}
