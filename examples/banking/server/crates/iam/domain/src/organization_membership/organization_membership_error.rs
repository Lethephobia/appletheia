use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    OrganizationMembershipCreateRejectionReason, OrganizationMembershipId,
    OrganizationMembershipRemoveRejectionReason, OrganizationMembershipRolesChangeRejectionReason,
    OrganizationMembershipStateError,
};

/// Describes why an `OrganizationMembership` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationMembershipError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationMembershipId>),

    #[error(transparent)]
    State(#[from] OrganizationMembershipStateError),

    #[error("organization membership is already created")]
    AlreadyCreated,

    #[error("organization membership creation rejected: {0:?}")]
    CreateRejected(OrganizationMembershipCreateRejectionReason),

    #[error("organization membership roles change rejected: {0:?}")]
    RolesChangeRejected(OrganizationMembershipRolesChangeRejectionReason),

    #[error("organization membership removal rejected: {0:?}")]
    RemoveRejected(OrganizationMembershipRemoveRejectionReason),
}
