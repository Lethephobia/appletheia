use appletheia::query;
use banking_iam_domain::OrganizationId;

/// Query parameters for organization information visible to members.
#[query(name = "organization_internal_info")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationInternalInfoQuery {
    pub organization_id: OrganizationId,
}
