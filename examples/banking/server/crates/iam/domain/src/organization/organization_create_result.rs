use super::OrganizationId;

/// Describes the domain outcome of an organization create request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OrganizationCreateResult {
    Created { organization_id: OrganizationId },
}
