mod currency_list_item_owner_user_part;
mod owned_account_list_owner_user_part;
mod owned_account_transaction_list_item_counterparty_account_owner_user_part;
mod owned_account_transaction_list_owner_user_part;
mod public_account_list_item_owner_user_part;

pub use currency_list_item_owner_user_part::CurrencyListItemOwnerUserPart;
pub use owned_account_list_owner_user_part::OwnedAccountListOwnerUserPart;
pub use owned_account_transaction_list_item_counterparty_account_owner_user_part::OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart;
pub use owned_account_transaction_list_owner_user_part::OwnedAccountTransactionListOwnerUserPart;
pub use public_account_list_item_owner_user_part::PublicAccountListItemOwnerUserPart;

impl_observation_part!(
    CurrencyListItemOwnerUserPart,
    "currency_list_item_owner_user",
    UserFragment,
    |part: &CurrencyListItemOwnerUserPart| part.id
);
impl_observation_part!(
    OwnedAccountListOwnerUserPart,
    "owned_account_list_owner_user",
    UserFragment,
    |part: &OwnedAccountListOwnerUserPart| part.id
);
impl_observation_part!(
    PublicAccountListItemOwnerUserPart,
    "public_account_list_item_owner_user",
    UserFragment,
    |part: &PublicAccountListItemOwnerUserPart| part.id
);
impl_observation_part!(
    OwnedAccountTransactionListOwnerUserPart,
    "owned_account_transaction_list_owner_user",
    UserFragment,
    |part: &OwnedAccountTransactionListOwnerUserPart| part.id
);
impl_observation_part!(
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
    "owned_account_transaction_list_item_counterparty_account_owner_user",
    UserFragment,
    |part: &OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart| part.id
);
use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use banking_iam_application::UserFragment;
