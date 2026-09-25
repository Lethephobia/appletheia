use appletheia::application::saga::SagaStep;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationInvitationSagaStep {
    CreateMembership,
}

impl SagaStep for OrganizationInvitationSagaStep {}
