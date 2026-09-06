use appletheia::domain::AggregateError;
use thiserror::Error;

use super::{
    OrganizationCreateRejectionReason, OrganizationDescriptionChangeRejectionReason,
    OrganizationDisplayNameChangeRejectionReason, OrganizationHandleChangeRejectionReason,
    OrganizationId, OrganizationOwnershipTransferRejectionReason,
    OrganizationPictureChangeRejectionReason, OrganizationRemoveRejectionReason,
    OrganizationStateError, OrganizationWebsiteUrlChangeRejectionReason,
};

/// Describes why an `Organization` aggregate operation failed.
#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<OrganizationId>),

    #[error(transparent)]
    State(#[from] OrganizationStateError),

    #[error("organization is already created")]
    AlreadyCreated,

    #[error("organization creation rejected: {0:?}")]
    CreateRejected(OrganizationCreateRejectionReason),
    #[error("organization ownership transfer rejected: {0:?}")]
    OwnershipTransferRejected(OrganizationOwnershipTransferRejectionReason),
    #[error("organization handle change rejected: {0:?}")]
    HandleChangeRejected(OrganizationHandleChangeRejectionReason),
    #[error("organization display name change rejected: {0:?}")]
    DisplayNameChangeRejected(OrganizationDisplayNameChangeRejectionReason),
    #[error("organization description change rejected: {0:?}")]
    DescriptionChangeRejected(OrganizationDescriptionChangeRejectionReason),
    #[error("organization website URL change rejected: {0:?}")]
    WebsiteUrlChangeRejected(OrganizationWebsiteUrlChangeRejectionReason),
    #[error("organization picture change rejected: {0:?}")]
    PictureChangeRejected(OrganizationPictureChangeRejectionReason),
    #[error("organization removal rejected: {0:?}")]
    RemoveRejected(OrganizationRemoveRejectionReason),
}
