use appletheia::event_payload;

use crate::{RoleId, UserId};

use super::{UserRoleAssignmentEventPayloadError, UserRoleAssignmentId};

/// Represents the domain events emitted by a `UserRoleAssignment` aggregate.
#[event_payload(error = UserRoleAssignmentEventPayloadError)]
pub enum UserRoleAssignmentEventPayload {
    Assigned {
        id: UserRoleAssignmentId,
        role_id: RoleId,
        user_id: UserId,
    },
    Revoked {
        role_id: RoleId,
        user_id: UserId,
    },
}

#[cfg(test)]
mod tests {
    use appletheia::domain::EventPayload;

    use crate::{RoleId, UserId};

    use super::{UserRoleAssignmentEventPayload, UserRoleAssignmentId};

    #[test]
    fn returns_stable_event_names() {
        assert_eq!(
            UserRoleAssignmentEventPayload::ASSIGNED,
            appletheia::domain::EventName::new("assigned")
        );
        assert_eq!(
            UserRoleAssignmentEventPayload::REVOKED,
            appletheia::domain::EventName::new("revoked")
        );
    }

    #[test]
    fn payload_name_matches_variant() {
        let payload = UserRoleAssignmentEventPayload::Assigned {
            id: UserRoleAssignmentId::new(),
            role_id: RoleId::admin(),
            user_id: UserId::new(),
        };

        assert_eq!(payload.name(), UserRoleAssignmentEventPayload::ASSIGNED);
    }
}
