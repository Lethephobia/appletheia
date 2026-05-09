use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

/// Projector specification for currency list item read models.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CurrencyListItemProjectorSpec;

impl ProjectorSpec for CurrencyListItemProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_list_item"),
        Subscription::AnyOf(&[
            EventSelector::new::<Currency>(CurrencyEventPayload::DEFINED),
            EventSelector::new::<Currency>(CurrencyEventPayload::OWNERSHIP_TRANSFERRED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SYMBOL_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::NAME_CHANGED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SUPPLY_INCREASED),
            EventSelector::new::<Currency>(CurrencyEventPayload::SUPPLY_DECREASED),
            EventSelector::new::<Currency>(CurrencyEventPayload::ACTIVATED),
            EventSelector::new::<Currency>(CurrencyEventPayload::DEACTIVATED),
            EventSelector::new::<Currency>(CurrencyEventPayload::REMOVED),
            EventSelector::new::<User>(UserEventPayload::REGISTERED),
            EventSelector::new::<User>(UserEventPayload::USERNAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<User>(UserEventPayload::PICTURE_CHANGED),
            EventSelector::new::<User>(UserEventPayload::REMOVED),
            EventSelector::new::<Organization>(OrganizationEventPayload::CREATED),
            EventSelector::new::<Organization>(OrganizationEventPayload::HANDLE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::DISPLAY_NAME_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::PICTURE_CHANGED),
            EventSelector::new::<Organization>(OrganizationEventPayload::REMOVED),
        ]),
    );
}
