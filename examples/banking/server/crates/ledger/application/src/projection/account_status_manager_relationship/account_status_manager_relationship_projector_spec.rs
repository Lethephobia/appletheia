use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountEventPayload};

/// Declares the subscription for the account status-manager relationship projector.
pub struct AccountStatusManagerRelationshipProjectorSpec;

impl ProjectorSpec for AccountStatusManagerRelationshipProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("account_status_manager_relationship"),
        Subscription::Only(&[EventSelector::new(
            Account::TYPE,
            AccountEventPayload::OPENED,
        )]),
    );
}
