use appletheia::query;
use banking_iam_domain::OrganizationId;

/// Query parameters for organization information visible to administrators.
#[query(name = "organization_management_info")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationManagementInfoQuery {
    pub organization_id: OrganizationId,
}
