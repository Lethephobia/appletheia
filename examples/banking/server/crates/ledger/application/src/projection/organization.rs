mod currency_list_item_owner_organization_part;
mod owned_account_list_owner_organization_part;
mod owned_account_transaction_list_item_counterparty_account_owner_organization_part;
mod owned_account_transaction_list_owner_organization_part;
mod public_account_list_item_owner_organization_part;

pub use currency_list_item_owner_organization_part::CurrencyListItemOwnerOrganizationPart;
pub use owned_account_list_owner_organization_part::OwnedAccountListOwnerOrganizationPart;
pub use owned_account_transaction_list_item_counterparty_account_owner_organization_part::OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart;
pub use owned_account_transaction_list_owner_organization_part::OwnedAccountTransactionListOwnerOrganizationPart;
pub use public_account_list_item_owner_organization_part::PublicAccountListItemOwnerOrganizationPart;

impl_observation_part!(
    CurrencyListItemOwnerOrganizationPart,
    "currency_list_item_owner_organization",
    OrganizationFragment,
    |part: &CurrencyListItemOwnerOrganizationPart| part.id
);
impl_observation_part!(
    OwnedAccountListOwnerOrganizationPart,
    "owned_account_list_owner_organization",
    OrganizationFragment,
    |part: &OwnedAccountListOwnerOrganizationPart| part.id
);
impl_observation_part!(
    PublicAccountListItemOwnerOrganizationPart,
    "public_account_list_item_owner_organization",
    OrganizationFragment,
    |part: &PublicAccountListItemOwnerOrganizationPart| part.id
);
impl_observation_part!(
    OwnedAccountTransactionListOwnerOrganizationPart,
    "owned_account_transaction_list_owner_organization",
    OrganizationFragment,
    |part: &OwnedAccountTransactionListOwnerOrganizationPart| part.id
);
impl_observation_part!(
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
    "owned_account_transaction_list_item_counterparty_account_owner_organization",
    OrganizationFragment,
    |part: &OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart| part.id
);
use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use banking_iam_application::OrganizationFragment;
