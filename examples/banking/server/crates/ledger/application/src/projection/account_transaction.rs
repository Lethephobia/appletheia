use appletheia::application::read_model::{
    ReadModelFragment, ReadModelPart, ReadModelPartName, ReadModelPartTree,
};

use crate::read_model::OwnedAccountTransactionListItemKind;

pub(super) use super::AccountFragment;
use super::{
    OwnedAccountTransactionListItemCounterpartyAccountPart,
    OwnedAccountTransactionListItemCurrencyPart,
};

mod account_transaction_currency_issuance_issued_record;
mod account_transaction_direction;
mod account_transaction_fragment;
mod account_transaction_fragment_insert;
mod account_transaction_fragment_kind;
mod account_transaction_fragment_projector;
mod account_transaction_fragment_projector_error;
mod account_transaction_fragment_projector_spec;
mod account_transaction_fragment_writer;
mod account_transaction_fragment_writer_error;
mod account_transaction_id;
mod account_transaction_status;
mod account_transaction_transfer_requested_record;
mod owned_account_transaction_list_item_part;

pub use account_transaction_currency_issuance_issued_record::AccountTransactionCurrencyIssuanceIssuedRecord;
pub use account_transaction_direction::AccountTransactionDirection;
pub use account_transaction_fragment::AccountTransactionFragment;
pub use account_transaction_fragment_insert::AccountTransactionFragmentInsert;
pub use account_transaction_fragment_kind::AccountTransactionFragmentKind;
pub use account_transaction_fragment_projector::AccountTransactionFragmentProjector;
pub use account_transaction_fragment_projector_error::AccountTransactionFragmentProjectorError;
pub use account_transaction_fragment_projector_spec::AccountTransactionFragmentProjectorSpec;
pub use account_transaction_fragment_writer::AccountTransactionFragmentWriter;
pub use account_transaction_fragment_writer_error::AccountTransactionFragmentWriterError;
pub use account_transaction_id::AccountTransactionId;
pub use account_transaction_status::AccountTransactionStatus;
pub use account_transaction_transfer_requested_record::AccountTransactionTransferRequestedRecord;
pub use owned_account_transaction_list_item_part::OwnedAccountTransactionListItemPart;

impl_composite_part!(
    OwnedAccountTransactionListItemPart,
    "owned_account_transaction_list_item",
    AccountTransactionFragment,
    |part: &OwnedAccountTransactionListItemPart| part.transaction_id,
    owned_account_transaction_list_item_parts
);

fn owned_account_transaction_list_item_parts(
    part: Option<&OwnedAccountTransactionListItemPart>,
) -> Vec<ReadModelPartTree> {
    let counterparty_account = part.and_then(|item| match &item.kind {
        OwnedAccountTransactionListItemKind::Transfer {
            counterparty_account,
            ..
        } => Some(counterparty_account),
        _ => None,
    });

    vec![
        ReadModelPartTree::field_with_explicit_route::<OwnedAccountTransactionListItemCurrencyPart>(
            "currency",
            part.map(|item| &item.currency),
        ),
        ReadModelPartTree::field_at_with_explicit_route::<
            OwnedAccountTransactionListItemCounterpartyAccountPart,
        >(
            &["kind", "counterparty_account"],
            counterparty_account.map(Box::as_ref),
        ),
    ]
}
