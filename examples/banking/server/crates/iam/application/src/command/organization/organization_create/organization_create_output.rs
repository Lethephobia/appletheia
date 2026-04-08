use banking_iam_domain::OrganizationId;
use serde::{Deserialize, Serialize};

/// The output returned after creating an organization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationCreateOutput {
    pub organization_id: OrganizationId,
}

impl OrganizationCreateOutput {
    /// Creates a new organization-create output.
    pub fn new(organization_id: OrganizationId) -> Self {
        Self { organization_id }
    }
}
