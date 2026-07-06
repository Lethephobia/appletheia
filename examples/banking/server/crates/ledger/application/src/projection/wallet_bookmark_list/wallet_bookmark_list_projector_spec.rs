use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_ledger_domain::wallet_bookmark::{WalletBookmark, WalletBookmarkEventPayload};

/// Projector specification for wallet bookmark list read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct WalletBookmarkListProjectorSpec;

impl ProjectorSpec for WalletBookmarkListProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("wallet_bookmark_list"),
        Subscription::AnyOf(&[
            EventSelector::new::<WalletBookmark>(WalletBookmarkEventPayload::REGISTERED),
            EventSelector::new::<WalletBookmark>(WalletBookmarkEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<WalletBookmark>(WalletBookmarkEventPayload::DESCRIPTION_CHANGED),
            EventSelector::new::<WalletBookmark>(WalletBookmarkEventPayload::REMOVED),
        ]),
    );
}
