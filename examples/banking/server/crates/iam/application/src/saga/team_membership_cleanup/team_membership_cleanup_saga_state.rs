use appletheia::application::saga::SagaState;
use banking_iam_domain::team_membership_cleanup::TeamMembershipCleanupId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMembershipCleanupSagaState {
    pub team_membership_cleanup_id: TeamMembershipCleanupId,
}

impl SagaState for TeamMembershipCleanupSagaState {}
