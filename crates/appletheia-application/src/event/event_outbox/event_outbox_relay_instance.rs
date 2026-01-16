use std::{fmt, fmt::Display, str::FromStr};

use super::{EventOutboxRelayInstanceError, EventOutboxRelayInstanceId, EventOutboxRelayProcessId};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EventOutboxRelayInstance {
    instance_id: EventOutboxRelayInstanceId,
    process_id: EventOutboxRelayProcessId,
}

impl EventOutboxRelayInstance {
    pub fn new(
        instance_id: EventOutboxRelayInstanceId,
        process_id: EventOutboxRelayProcessId,
    ) -> Self {
        Self {
            instance_id,
            process_id,
        }
    }

    pub fn instance_id(&self) -> &EventOutboxRelayInstanceId {
        &self.instance_id
    }

    pub fn process_id(&self) -> EventOutboxRelayProcessId {
        self.process_id
    }
}

impl Display for EventOutboxRelayInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.instance_id, self.process_id)
    }
}

impl FromStr for EventOutboxRelayInstance {
    type Err = EventOutboxRelayInstanceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.matches(':').count() != 1 {
            return Err(EventOutboxRelayInstanceError::MissingSeparator);
        }
        let (instance_raw, process_raw) = s
            .split_once(':')
            .ok_or(EventOutboxRelayInstanceError::MissingSeparator)?;

        if instance_raw.is_empty() {
            return Err(EventOutboxRelayInstanceError::EmptyInstanceId);
        }
        if process_raw.is_empty() {
            return Err(EventOutboxRelayInstanceError::EmptyProcessId);
        }

        let instance_id = EventOutboxRelayInstanceId::new(instance_raw.to_string())?;
        let process_num = process_raw
            .parse::<u32>()
            .map_err(EventOutboxRelayInstanceError::InvalidProcessId)?;
        let process_id = EventOutboxRelayProcessId::from(process_num);

        Ok(EventOutboxRelayInstance::new(instance_id, process_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_and_parses_round_trip() {
        let instance = EventOutboxRelayInstanceId::new("instance-1".to_string()).unwrap();
        let process = EventOutboxRelayProcessId::new(42);
        let owner = EventOutboxRelayInstance::new(instance.clone(), process);

        let text = owner.to_string();
        let parsed = EventOutboxRelayInstance::from_str(&text).unwrap();

        assert_eq!(parsed.instance_id(), &instance);
        assert_eq!(parsed.process_id(), process);
    }

    #[test]
    fn fails_without_separator() {
        let err = EventOutboxRelayInstance::from_str("noseparator").expect_err("should fail");
        assert!(matches!(
            err,
            EventOutboxRelayInstanceError::MissingSeparator
        ));
    }

    #[test]
    fn fails_on_empty_instance_id() {
        let err = EventOutboxRelayInstance::from_str(":123").expect_err("should fail");
        assert!(matches!(
            err,
            EventOutboxRelayInstanceError::EmptyInstanceId
        ));
    }

    #[test]
    fn fails_on_empty_process_id() {
        let err = EventOutboxRelayInstance::from_str("instance-1:").expect_err("should fail");
        assert!(matches!(err, EventOutboxRelayInstanceError::EmptyProcessId));
    }
}
