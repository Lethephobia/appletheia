use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_ledger_domain::account::{Account, AccountEventPayload};

/// Projector specification for account fragments.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountFragmentProjectorSpec;

impl ProjectorSpec for AccountFragmentProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("account_fragment"),
        Subscription::AnyOf(&[
            EventSelector::new::<Account>(AccountEventPayload::OPENED),
            EventSelector::new::<Account>(AccountEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new::<Account>(AccountEventPayload::NAME_CHANGED),
            EventSelector::new::<Account>(AccountEventPayload::DEPOSITED),
            EventSelector::new::<Account>(AccountEventPayload::WITHDRAWN),
            EventSelector::new::<Account>(AccountEventPayload::FUNDS_RESERVED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_RELEASED),
            EventSelector::new::<Account>(AccountEventPayload::RESERVED_FUNDS_COMMITTED),
            EventSelector::new::<Account>(AccountEventPayload::FROZEN),
            EventSelector::new::<Account>(AccountEventPayload::THAWED),
            EventSelector::new::<Account>(AccountEventPayload::CLOSED),
        ]),
    );
}
