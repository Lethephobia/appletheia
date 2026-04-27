use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// Stores context for the organization picture saga.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationPictureSagaContext {
    pub organization_id: OrganizationId,
}

impl OrganizationPictureSagaContext {
    pub fn new(organization_id: OrganizationId) -> Self {
        Self { organization_id }
    }
}
