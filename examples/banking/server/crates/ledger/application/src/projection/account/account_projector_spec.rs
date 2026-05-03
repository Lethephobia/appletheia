use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

/// Declares the subscription for the account view projector.
pub struct AccountProjectorSpec;

impl ProjectorSpec for AccountProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("account"),
        Subscription::AnyOf(&[
            EventSelector::new(Account::TYPE, AccountEventPayload::OPENED),
            EventSelector::new(Account::TYPE, AccountEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new(Account::TYPE, AccountEventPayload::NAME_CHANGED),
            EventSelector::new(Account::TYPE, AccountEventPayload::DEPOSITED),
            EventSelector::new(Account::TYPE, AccountEventPayload::WITHDRAWN),
            EventSelector::new(Account::TYPE, AccountEventPayload::FUNDS_RESERVED),
            EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_RELEASED),
            EventSelector::new(Account::TYPE, AccountEventPayload::RESERVED_FUNDS_COMMITTED),
            EventSelector::new(Account::TYPE, AccountEventPayload::FROZEN),
            EventSelector::new(Account::TYPE, AccountEventPayload::THAWED),
            EventSelector::new(Account::TYPE, AccountEventPayload::CLOSED),
        ]),
    );
}
