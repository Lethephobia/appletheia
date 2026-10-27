use super::ReadModelWatchRetryOptions;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ReadModelWatchConfig {
    pub retry_options: ReadModelWatchRetryOptions,
}
