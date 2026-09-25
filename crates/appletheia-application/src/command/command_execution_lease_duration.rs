use chrono::Duration;

/// Bounds how long one command execution attempt holds its lease.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CommandExecutionLeaseDuration(Duration);

impl CommandExecutionLeaseDuration {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Duration {
        self.0
    }
}

impl Default for CommandExecutionLeaseDuration {
    fn default() -> Self {
        Self(Duration::minutes(5))
    }
}

impl From<Duration> for CommandExecutionLeaseDuration {
    fn from(value: Duration) -> Self {
        Self::new(value)
    }
}

impl From<CommandExecutionLeaseDuration> for Duration {
    fn from(value: CommandExecutionLeaseDuration) -> Self {
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_five_minutes() {
        assert_eq!(
            CommandExecutionLeaseDuration::default().value(),
            Duration::minutes(5)
        );
    }

    #[test]
    fn conversions_round_trip() {
        let duration = Duration::seconds(30);
        let wrapped = CommandExecutionLeaseDuration::from(duration);
        let restored = Duration::from(wrapped);

        assert_eq!(restored, duration);
    }
}
