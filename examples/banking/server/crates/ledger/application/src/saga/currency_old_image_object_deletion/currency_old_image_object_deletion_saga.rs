use appletheia::application::event::EventEnvelope;
use appletheia::application::saga::{Saga, SagaInstance, SagaSpec};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use crate::command::CurrencyImageObjectDeleteCommand;

use super::{
    CurrencyOldImageObjectDeletionSagaError, CurrencyOldImageObjectDeletionSagaSpec,
    CurrencyOldImageObjectDeletionSagaState, CurrencyOldImageObjectDeletionSagaStatus,
};

/// Coordinates old currency image object deletion after image changes.
pub struct CurrencyOldImageObjectDeletionSaga;

impl Saga for CurrencyOldImageObjectDeletionSaga {
    type Spec = CurrencyOldImageObjectDeletionSagaSpec;
    type Error = CurrencyOldImageObjectDeletionSagaError;

    fn on_event(
        &self,
        instance: &mut SagaInstance<<Self::Spec as SagaSpec>::State>,
        event_envelope: &EventEnvelope,
    ) -> Result<(), Self::Error> {
        let event = event_envelope.try_into_domain_event::<Currency>()?;
        let CurrencyEventPayload::ImageChanged { old_image, .. } = event.payload() else {
            return Err(CurrencyOldImageObjectDeletionSagaError::UnexpectedEvent);
        };

        let state = CurrencyOldImageObjectDeletionSagaState::new(event.aggregate_id());
        *instance.state_mut() = Some(state);
        let Some(object_name) = old_image
            .as_ref()
            .and_then(|image| image.as_object_name())
            .cloned()
        else {
            instance.state_required_mut()?.status =
                CurrencyOldImageObjectDeletionSagaStatus::Skipped;
            instance.succeed();
            return Ok(());
        };

        instance.append_command(
            event_envelope,
            &CurrencyImageObjectDeleteCommand { object_name },
        )?;
        instance.succeed();

        Ok(())
    }
}
