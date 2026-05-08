use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosureEventPayload,
};

use super::OwnedAccountClosureSagaState;

/// Declares the descriptor and state for the owned account closure saga.
pub struct OwnedAccountClosureSagaSpec;

impl SagaSpec for OwnedAccountClosureSagaSpec {
    type State = OwnedAccountClosureSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("owned_account_closure"),
        SagaStartEvents::new(&[
            EventSelector::new(User::TYPE, UserEventPayload::REMOVED),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::REMOVED),
        ]),
        Subscription::AnyOf(&[
            EventSelector::new(User::TYPE, UserEventPayload::REMOVED),
            EventSelector::new(Organization::TYPE, OrganizationEventPayload::REMOVED),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::REQUESTED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::PAGE_LOADED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::PAGE_LOAD_REJECTED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_RECORDED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_RECORD_REJECTED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_REJECTION_RECORDED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::ACCOUNT_CLOSE_REJECTION_RECORD_REJECTED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::COMPLETED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::COMPLETE_REJECTED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::FAILED,
            ),
            EventSelector::new(
                OwnedAccountClosure::TYPE,
                OwnedAccountClosureEventPayload::FAIL_REJECTED,
            ),
            EventSelector::new(Account::TYPE, AccountEventPayload::CLOSED),
            EventSelector::new(Account::TYPE, AccountEventPayload::CLOSE_REJECTED),
        ]),
    );
}
