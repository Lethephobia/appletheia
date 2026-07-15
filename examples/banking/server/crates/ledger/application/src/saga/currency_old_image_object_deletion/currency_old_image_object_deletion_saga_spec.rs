use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::saga::{SagaDescriptor, SagaName, SagaSpec, SagaStartEvents};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::CurrencyOldImageObjectDeletionSagaState;

/// Declares the descriptor and state for the currency old image object deletion saga.
pub struct CurrencyOldImageObjectDeletionSagaSpec;

impl SagaSpec for CurrencyOldImageObjectDeletionSagaSpec {
    type State = CurrencyOldImageObjectDeletionSagaState;

    const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
        SagaName::new("currency_old_image_object_deletion"),
        SagaStartEvents::new(&[EventSelector::new::<Currency>(
            CurrencyEventPayload::IMAGE_CHANGED,
        )]),
        Subscription::One(&EventSelector::new::<Currency>(
            CurrencyEventPayload::IMAGE_CHANGED,
        )),
    );
}
