use appletheia::command;
use banking_iam_domain::{OrganizationId, OrganizationProfile};
use serde::{Deserialize, Serialize};

/// Replaces an organization's profile.
#[command(name = "organization_profile_change")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationProfileChangeCommand {
    pub organization_id: OrganizationId,
    pub profile: OrganizationProfile,
}
