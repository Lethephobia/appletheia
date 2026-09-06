use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureEventPayload,
};

/// Declares the descriptor for the owned account closure saga.
pub struct OwnedAccountClosureSagaSpec;

impl SagaSpec for OwnedAccountClosureSagaSpec {
    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("owned_account_closure"),
        SagaStartEvents::new(&[
            EventSelector::new::<User>(UserEventPayload::REMOVED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
        Subscription::AnyOf(&[
            EventSelector::new::<User>(UserEventPayload::REMOVED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
            EventSelector::new::<OwnedAccountClosure>(OwnedAccountClosureEventPayload::REQUESTED),
            EventSelector::new::<OwnedAccountClosure>(OwnedAccountClosureEventPayload::PAGE_LOADED),
            EventSelector::new::<OwnedAccountClosure>(
                OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_RECORDED,
            ),
            EventSelector::new::<OwnedAccountClosure>(
                OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_REJECTION_RECORDED,
            ),
            EventSelector::new::<OwnedAccountClosure>(OwnedAccountClosureEventPayload::COMPLETED),
            EventSelector::new::<OwnedAccountClosure>(OwnedAccountClosureEventPayload::FAILED),
            EventSelector::new::<Account>(AccountEventPayload::CLOSED),
        ]),
    );
}
