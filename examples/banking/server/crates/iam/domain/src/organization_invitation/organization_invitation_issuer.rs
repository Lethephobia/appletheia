use serde::{Deserialize, Serialize};

use crate::UserId;

/// Identifies who issued an organization invitation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum OrganizationInvitationIssuer {
    User(UserId),
    System,
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use uuid::Uuid;

    use super::OrganizationInvitationIssuer;
    use crate::UserId;

    #[test]
    fn user_variant_preserves_user_id() {
        let user_id = UserId::try_from_uuid(Uuid::now_v7()).expect("user id should be accepted");
        let issuer = OrganizationInvitationIssuer::User(user_id);

        assert_eq!(issuer, OrganizationInvitationIssuer::User(user_id));
    }

    #[test]
    fn system_variant_is_stable() {
        assert_eq!(
            OrganizationInvitationIssuer::System,
            OrganizationInvitationIssuer::System
        );
    }
}
