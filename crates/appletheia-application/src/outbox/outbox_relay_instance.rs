use std::{fmt, fmt::Display, str::FromStr};

use super::{OutboxRelayInstanceError, OutboxRelayInstanceId, OutboxRelayProcessId};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OutboxRelayInstance {
    instance_id: OutboxRelayInstanceId,
    process_id: OutboxRelayProcessId,
}

impl OutboxRelayInstance {
    pub fn new(instance_id: OutboxRelayInstanceId, process_id: OutboxRelayProcessId) -> Self {
        Self {
            instance_id,
            process_id,
        }
    }

    pub fn instance_id(&self) -> &OutboxRelayInstanceId {
        &self.instance_id
    }

    pub fn process_id(&self) -> OutboxRelayProcessId {
        self.process_id
    }
}

impl Display for OutboxRelayInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.instance_id, self.process_id)
    }
}

impl FromStr for OutboxRelayInstance {
    type Err = OutboxRelayInstanceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.matches(':').count() != 1 {
            return Err(OutboxRelayInstanceError::MissingSeparator);
        }
        let (instance_raw, process_raw) = s
            .split_once(':')
            .ok_or(OutboxRelayInstanceError::MissingSeparator)?;

        if instance_raw.is_empty() {
            return Err(OutboxRelayInstanceError::EmptyInstanceId);
        }
        if process_raw.is_empty() {
            return Err(OutboxRelayInstanceError::EmptyProcessId);
        }

        let instance_id = OutboxRelayInstanceId::new(instance_raw.to_string())?;
        let process_num = process_raw
            .parse::<u32>()
            .map_err(OutboxRelayInstanceError::InvalidProcessId)?;
        let process_id = OutboxRelayProcessId::from(process_num);

        Ok(OutboxRelayInstance::new(instance_id, process_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_and_parses_round_trip() {
        let instance = OutboxRelayInstanceId::new("instance-1".to_string()).unwrap();
        let process = OutboxRelayProcessId::new(42);
        let owner = OutboxRelayInstance::new(instance.clone(), process);

        let text = owner.to_string();
        let parsed = OutboxRelayInstance::from_str(&text).unwrap();

        assert_eq!(parsed.instance_id(), &instance);
        assert_eq!(parsed.process_id(), process);
    }

    #[test]
    fn fails_without_separator() {
        let err = OutboxRelayInstance::from_str("noseparator").expect_err("should fail");
        assert!(matches!(err, OutboxRelayInstanceError::MissingSeparator));
    }

    #[test]
    fn fails_on_empty_instance_id() {
        let err = OutboxRelayInstance::from_str(":123").expect_err("should fail");
        assert!(matches!(err, OutboxRelayInstanceError::EmptyInstanceId));
    }

    #[test]
    fn fails_on_empty_process_id() {
        let err = OutboxRelayInstance::from_str("instance-1:").expect_err("should fail");
        assert!(matches!(err, OutboxRelayInstanceError::EmptyProcessId));
    }
}
