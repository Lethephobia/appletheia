use appletheia::application::read_model::{
    ReadModelFragment, ReadModelPart, ReadModelPartName, ReadModelPartTree,
};

use crate::read_model::{
    OwnedAccountTransactionListItemCounterpartyAccountOwner, PublicAccountListItemOwner,
};

pub(super) use super::{CurrencyFragment, FragmentOwner};
use super::{
    OwnedAccountListItemCurrencyPart,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
    OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
    PublicAccountListItemCurrencyPart, PublicAccountListItemOwnerOrganizationPart,
    PublicAccountListItemOwnerUserPart,
};

mod account_fragment;
mod account_fragment_projector;
mod account_fragment_projector_error;
mod account_fragment_projector_spec;
mod account_fragment_upsert;
mod account_fragment_writer;
mod account_fragment_writer_error;
mod materialized_account_status;
mod materialized_account_status_error;
mod owned_account_list_item_part;
mod owned_account_transaction_list_item_counterparty_account_part;
mod public_account_list_item_part;

pub use account_fragment::AccountFragment;
pub use account_fragment_projector::AccountFragmentProjector;
pub use account_fragment_projector_error::AccountFragmentProjectorError;
pub use account_fragment_projector_spec::AccountFragmentProjectorSpec;
pub use account_fragment_upsert::AccountFragmentUpsert;
pub use account_fragment_writer::AccountFragmentWriter;
pub use account_fragment_writer_error::AccountFragmentWriterError;
pub use materialized_account_status::MaterializedAccountStatus;
pub use materialized_account_status_error::MaterializedAccountStatusError;
pub use owned_account_list_item_part::OwnedAccountListItemPart;
pub use owned_account_transaction_list_item_counterparty_account_part::OwnedAccountTransactionListItemCounterpartyAccountPart;
pub use public_account_list_item_part::PublicAccountListItemPart;

impl_composite_part!(
    OwnedAccountListItemPart,
    "owned_account_list_item",
    AccountFragment,
    |part: &OwnedAccountListItemPart| part.account_id,
    owned_account_list_item_parts
);
impl_composite_part!(
    PublicAccountListItemPart,
    "public_account_list_item",
    AccountFragment,
    |part: &PublicAccountListItemPart| part.account_id,
    public_account_list_item_parts
);
impl_composite_part!(
    OwnedAccountTransactionListItemCounterpartyAccountPart,
    "owned_account_transaction_list_item_counterparty_account",
    AccountFragment,
    |part: &OwnedAccountTransactionListItemCounterpartyAccountPart| part.id,
    counterparty_account_parts
);

fn owned_account_list_item_parts(
    part: Option<&OwnedAccountListItemPart>,
) -> Vec<ReadModelPartTree> {
    vec![ReadModelPartTree::field_with_explicit_route::<
        OwnedAccountListItemCurrencyPart,
    >("currency", part.map(|item| &item.currency))]
}

fn public_account_list_item_parts(
    part: Option<&PublicAccountListItemPart>,
) -> Vec<ReadModelPartTree> {
    let owner = part.map(|item| &item.owner);
    let user = owner.and_then(|owner| match owner {
        PublicAccountListItemOwner::User(user) => Some(user),
        PublicAccountListItemOwner::Organization(_) => None,
    });
    let organization = owner.and_then(|owner| match owner {
        PublicAccountListItemOwner::User(_) => None,
        PublicAccountListItemOwner::Organization(organization) => Some(organization),
    });

    vec![
        ReadModelPartTree::field_with_explicit_route::<PublicAccountListItemCurrencyPart>(
            "currency",
            part.map(|item| &item.currency),
        ),
        ReadModelPartTree::field_with_explicit_route::<PublicAccountListItemOwnerUserPart>(
            "owner", user,
        ),
        ReadModelPartTree::field_with_explicit_route::<PublicAccountListItemOwnerOrganizationPart>(
            "owner",
            organization,
        ),
    ]
}

fn counterparty_account_parts(
    part: Option<&OwnedAccountTransactionListItemCounterpartyAccountPart>,
) -> Vec<ReadModelPartTree> {
    let owner = part.map(|account| &account.owner);
    let user = owner.and_then(|owner| match owner {
        OwnedAccountTransactionListItemCounterpartyAccountOwner::User(user) => Some(user),
        OwnedAccountTransactionListItemCounterpartyAccountOwner::Organization(_) => None,
    });
    let organization = owner.and_then(|owner| match owner {
        OwnedAccountTransactionListItemCounterpartyAccountOwner::User(_) => None,
        OwnedAccountTransactionListItemCounterpartyAccountOwner::Organization(organization) => {
            Some(organization)
        }
    });

    vec![
        ReadModelPartTree::field_with_explicit_route::<
            OwnedAccountTransactionListItemCounterpartyAccountOwnerUserPart,
        >("owner", user),
        ReadModelPartTree::field_with_explicit_route::<
            OwnedAccountTransactionListItemCounterpartyAccountOwnerOrganizationPart,
        >("owner", organization),
    ]
}
