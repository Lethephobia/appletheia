use appletheia::application::saga::SagaState;
use banking_iam_domain::OrganizationInvitationId;
use serde::{Deserialize, Serialize};

/// Stores the progress of the organization invitation saga.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInvitationSagaState {
    pub organization_invitation_id: Option<OrganizationInvitationId>,
}

impl SagaState for OrganizationInvitationSagaState {}
