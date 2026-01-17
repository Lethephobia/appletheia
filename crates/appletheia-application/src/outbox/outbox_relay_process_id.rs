use std::{fmt, fmt::Display};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct OutboxRelayProcessId(u32);

impl OutboxRelayProcessId {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl From<u32> for OutboxRelayProcessId {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl Display for OutboxRelayProcessId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_round_trips() {
        let pid = OutboxRelayProcessId::new(1234);
        assert_eq!(pid.value(), 1234);

        let from_u32 = OutboxRelayProcessId::from(5678);
        assert_eq!(from_u32.value(), 5678);
    }
}
