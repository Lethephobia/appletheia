use appletheia::application::saga::SagaState;
use banking_iam_domain::{OrganizationId, OrganizationInvitationId, UserId};
use serde::{Deserialize, Serialize};

/// Stores the progress of the organization invitation saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationSagaState {
    pub organization_invitation_id: Option<OrganizationInvitationId>,
    pub organization_id: Option<OrganizationId>,
    pub invitee_id: Option<UserId>,
}

impl SagaState for OrganizationInvitationSagaState {}
