use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

/// Declares the subscription for the account owner relationship projector.
pub struct AccountOwnerRelationshipProjectorSpec;

impl ProjectorSpec for AccountOwnerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("account_owner_relationship"),
        Subscription::AnyOf(&[EventSelector::new(
            Account::TYPE,
            AccountEventPayload::OPENED,
        )]),
    );
}
