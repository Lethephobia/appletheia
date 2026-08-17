use crate::read_model::CurrencyListItemOwner;
use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName, ReadModelPartTree,
};

pub(super) use super::FragmentOwner;
use super::{CurrencyListItemOwnerOrganizationPart, CurrencyListItemOwnerUserPart};

mod currency_fragment;
mod currency_fragment_projector;
mod currency_fragment_projector_error;
mod currency_fragment_projector_spec;
mod currency_fragment_upsert;
mod currency_fragment_writer;
mod currency_fragment_writer_error;
mod currency_list_item_part;
mod materialized_currency_status;
mod materialized_currency_status_error;
mod owned_account_list_item_currency_part;
mod owned_account_transaction_list_item_currency_part;
mod public_account_list_item_currency_part;

pub use currency_fragment::CurrencyFragment;
pub use currency_fragment_projector::CurrencyFragmentProjector;
pub use currency_fragment_projector_error::CurrencyFragmentProjectorError;
pub use currency_fragment_projector_spec::CurrencyFragmentProjectorSpec;
pub use currency_fragment_upsert::CurrencyFragmentUpsert;
pub use currency_fragment_writer::CurrencyFragmentWriter;
pub use currency_fragment_writer_error::CurrencyFragmentWriterError;
pub use currency_list_item_part::CurrencyListItemPart;
pub use materialized_currency_status::MaterializedCurrencyStatus;
pub use materialized_currency_status_error::MaterializedCurrencyStatusError;
pub use owned_account_list_item_currency_part::OwnedAccountListItemCurrencyPart;
pub use owned_account_transaction_list_item_currency_part::OwnedAccountTransactionListItemCurrencyPart;
pub use public_account_list_item_currency_part::PublicAccountListItemCurrencyPart;

impl_composite_part!(
    CurrencyListItemPart,
    "currency_list_item",
    CurrencyFragment,
    |part: &CurrencyListItemPart| part.currency_id,
    currency_list_item_parts
);
impl_observation_part!(
    OwnedAccountListItemCurrencyPart,
    "owned_account_list_item_currency",
    CurrencyFragment,
    |part: &OwnedAccountListItemCurrencyPart| part.id
);
impl_observation_part!(
    PublicAccountListItemCurrencyPart,
    "public_account_list_item_currency",
    CurrencyFragment,
    |part: &PublicAccountListItemCurrencyPart| part.id
);
impl_part!(
    OwnedAccountTransactionListItemCurrencyPart,
    "owned_account_transaction_list_item_currency",
    CurrencyFragment,
    |part: &OwnedAccountTransactionListItemCurrencyPart| part.id
);

fn currency_list_item_parts(part: Option<&CurrencyListItemPart>) -> Vec<ReadModelPartTree> {
    let owner = part.map(|item| &item.owner);
    let user = owner.and_then(|owner| match owner {
        CurrencyListItemOwner::User(user) => Some(user),
        CurrencyListItemOwner::Organization(_) => None,
    });
    let organization = owner.and_then(|owner| match owner {
        CurrencyListItemOwner::User(_) => None,
        CurrencyListItemOwner::Organization(organization) => Some(organization),
    });

    vec![
        ReadModelPartTree::field_with_explicit_route::<CurrencyListItemOwnerUserPart>(
            "owner", user,
        ),
        ReadModelPartTree::field_with_explicit_route::<CurrencyListItemOwnerOrganizationPart>(
            "owner",
            organization,
        ),
    ]
}
