use appletheia::application::read_model::{ReadModelFragment, ReadModelPart, ReadModelPartName};

pub(super) use super::FragmentOwner;

mod wallet_bookmark_fragment;
mod wallet_bookmark_fragment_projector;
mod wallet_bookmark_fragment_projector_error;
mod wallet_bookmark_fragment_projector_spec;
mod wallet_bookmark_fragment_upsert;
mod wallet_bookmark_fragment_writer;
mod wallet_bookmark_fragment_writer_error;
mod wallet_bookmark_list_item_part;

pub use wallet_bookmark_fragment::WalletBookmarkFragment;
pub use wallet_bookmark_fragment_projector::WalletBookmarkFragmentProjector;
pub use wallet_bookmark_fragment_projector_error::WalletBookmarkFragmentProjectorError;
pub use wallet_bookmark_fragment_projector_spec::WalletBookmarkFragmentProjectorSpec;
pub use wallet_bookmark_fragment_upsert::WalletBookmarkFragmentUpsert;
pub use wallet_bookmark_fragment_writer::WalletBookmarkFragmentWriter;
pub use wallet_bookmark_fragment_writer_error::WalletBookmarkFragmentWriterError;
pub use wallet_bookmark_list_item_part::WalletBookmarkListItemPart;

impl_part!(
    WalletBookmarkListItemPart,
    "wallet_bookmark_list_item",
    WalletBookmarkFragment,
    |part: &WalletBookmarkListItemPart| part.wallet_bookmark_id
);
