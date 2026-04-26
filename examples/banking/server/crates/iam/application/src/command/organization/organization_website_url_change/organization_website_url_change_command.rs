use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationWebsiteUrl};
use serde::{Deserialize, Serialize};

/// Changes an organization's website URL.
#[command(name = "organization_website_url_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationWebsiteUrlChangeCommand {
    pub organization_id: OrganizationId,
    pub website_url: Option<OrganizationWebsiteUrl>,
}
