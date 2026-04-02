use appletheia::command;
use banking_iam_domain::OrganizationName;
use serde::{Deserialize, Serialize};

/// Creates a new organization.
#[command(name = "organization_create")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationCreateCommand {
    pub name: OrganizationName,
}
