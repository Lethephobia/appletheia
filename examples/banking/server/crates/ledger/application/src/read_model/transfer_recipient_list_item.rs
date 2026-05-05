use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};

mod transfer_recipient_list_item_account;
mod transfer_recipient_list_item_account_status;
mod transfer_recipient_list_item_currency;
mod transfer_recipient_list_item_cursor;
mod transfer_recipient_list_item_reader;
mod transfer_recipient_list_item_reader_error;
mod transfer_recipient_list_item_sort_key;
mod transfer_recipient_list_item_user_status;
mod transfer_recipient_list_item_writer;
mod transfer_recipient_list_item_writer_error;

pub use transfer_recipient_list_item_account::TransferRecipientListItemAccount;
pub use transfer_recipient_list_item_account_status::TransferRecipientListItemAccountStatus;
pub use transfer_recipient_list_item_currency::TransferRecipientListItemCurrency;
pub use transfer_recipient_list_item_cursor::TransferRecipientListItemCursor;
pub use transfer_recipient_list_item_reader::TransferRecipientListItemReader;
pub use transfer_recipient_list_item_reader_error::TransferRecipientListItemReaderError;
pub use transfer_recipient_list_item_sort_key::TransferRecipientListItemSortKey;
pub use transfer_recipient_list_item_user_status::TransferRecipientListItemUserStatus;
pub use transfer_recipient_list_item_writer::TransferRecipientListItemWriter;
pub use transfer_recipient_list_item_writer_error::TransferRecipientListItemWriterError;

/// Read model for one transfer recipient user list row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecipientListItem {
    pub user_id: UserId,
    pub username: Option<Username>,
    pub display_name: Option<UserDisplayName>,
    pub picture: Option<UserPictureRef>,
    pub accounts: Vec<TransferRecipientListItemAccount>,
    pub created_at: EventOccurredAt,
}
