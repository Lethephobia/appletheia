mod currency_registrar_description;
mod currency_registrar_description_error;
mod currency_registrar_display_name;
mod currency_registrar_display_name_error;
mod currency_registrar_error;
mod currency_registrar_event_payload;
mod currency_registrar_event_payload_error;
mod currency_registrar_handle;
mod currency_registrar_handle_error;
mod currency_registrar_id;
mod currency_registrar_state;
mod currency_registrar_state_error;

pub use currency_registrar_description::CurrencyRegistrarDescription;
pub use currency_registrar_description_error::CurrencyRegistrarDescriptionError;
pub use currency_registrar_display_name::CurrencyRegistrarDisplayName;
pub use currency_registrar_display_name_error::CurrencyRegistrarDisplayNameError;
pub use currency_registrar_error::CurrencyRegistrarError;
pub use currency_registrar_event_payload::CurrencyRegistrarEventPayload;
pub use currency_registrar_event_payload_error::CurrencyRegistrarEventPayloadError;
pub use currency_registrar_handle::CurrencyRegistrarHandle;
pub use currency_registrar_handle_error::CurrencyRegistrarHandleError;
pub use currency_registrar_id::CurrencyRegistrarId;
pub use currency_registrar_state::CurrencyRegistrarState;
pub use currency_registrar_state_error::CurrencyRegistrarStateError;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

/// Represents an authorization boundary for registering and managing currencies.
#[aggregate(type = "currency_registrar", error = CurrencyRegistrarError)]
pub struct CurrencyRegistrar {
    core: AggregateCore<CurrencyRegistrarId, CurrencyRegistrarState, CurrencyRegistrarEventPayload>,
}

impl CurrencyRegistrar {
    pub fn handle(&self) -> Result<&CurrencyRegistrarHandle, CurrencyRegistrarError> {
        Ok(&self.state_required()?.handle)
    }

    pub fn display_name(&self) -> Result<&CurrencyRegistrarDisplayName, CurrencyRegistrarError> {
        Ok(&self.state_required()?.display_name)
    }

    pub fn description(
        &self,
    ) -> Result<Option<&CurrencyRegistrarDescription>, CurrencyRegistrarError> {
        Ok(self.state_required()?.description.as_ref())
    }

    /// Creates the registrar.
    pub fn create(
        &mut self,
        handle: CurrencyRegistrarHandle,
        display_name: CurrencyRegistrarDisplayName,
    ) -> Result<(), CurrencyRegistrarError> {
        if self.state().is_some() {
            return Err(CurrencyRegistrarError::AlreadyCreated);
        }

        self.append_event(CurrencyRegistrarEventPayload::Created {
            handle,
            display_name,
        })?;
        Ok(())
    }

    pub fn change_handle(
        &mut self,
        handle: CurrencyRegistrarHandle,
    ) -> Result<(), CurrencyRegistrarError> {
        self.state_required()?;
        self.append_event(CurrencyRegistrarEventPayload::HandleChanged { handle })?;
        Ok(())
    }

    pub fn change_display_name(
        &mut self,
        display_name: CurrencyRegistrarDisplayName,
    ) -> Result<(), CurrencyRegistrarError> {
        self.state_required()?;
        self.append_event(CurrencyRegistrarEventPayload::DisplayNameChanged { display_name })?;
        Ok(())
    }

    pub fn set_description(
        &mut self,
        description: Option<CurrencyRegistrarDescription>,
    ) -> Result<(), CurrencyRegistrarError> {
        self.state_required()?;
        self.append_event(CurrencyRegistrarEventPayload::DescriptionSet { description })?;
        Ok(())
    }
}

impl AggregateApply<CurrencyRegistrarEventPayload, CurrencyRegistrarError> for CurrencyRegistrar {
    fn apply(
        &mut self,
        payload: &CurrencyRegistrarEventPayload,
    ) -> Result<(), CurrencyRegistrarError> {
        match payload {
            CurrencyRegistrarEventPayload::Created {
                handle,
                display_name,
            } => {
                self.set_state(Some(CurrencyRegistrarState {
                    handle: handle.clone(),
                    display_name: display_name.clone(),
                    description: None,
                }));
            }
            CurrencyRegistrarEventPayload::HandleChanged { handle } => {
                self.state_required_mut()?.handle = handle.clone();
            }
            CurrencyRegistrarEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = display_name.clone();
            }
            CurrencyRegistrarEventPayload::DescriptionSet { description } => {
                self.state_required_mut()?.description = description.clone();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateApply};

    use super::{CurrencyRegistrar, CurrencyRegistrarDisplayName, CurrencyRegistrarHandle};

    #[test]
    fn create_initializes_the_registrar() {
        let mut registrar = CurrencyRegistrar::new();

        registrar
            .create(
                CurrencyRegistrarHandle::try_from("example").expect("handle should be valid"),
                CurrencyRegistrarDisplayName::try_from("Example")
                    .expect("display name should be valid"),
            )
            .expect("registrar should be created");

        assert!(registrar.state().is_some());
    }

    #[test]
    fn optional_fields_are_set_separately_and_replayable() {
        let mut currency_registrar = CurrencyRegistrar::new();
        currency_registrar
            .create(
                CurrencyRegistrarHandle::try_from("example").unwrap(),
                CurrencyRegistrarDisplayName::try_from("Example").unwrap(),
            )
            .unwrap();
        assert_eq!(currency_registrar.uncommitted_events().len(), 1);
        assert_eq!(currency_registrar.description().unwrap(), None);
        let value = super::CurrencyRegistrarDescription::try_from("Registrar").unwrap();
        currency_registrar
            .set_description(Some(value.clone()))
            .unwrap();
        assert_eq!(currency_registrar.description().unwrap(), Some(&value));
        currency_registrar.set_description(None).unwrap();
        currency_registrar.set_description(None).unwrap();
        assert_eq!(currency_registrar.description().unwrap(), None);
        assert_eq!(currency_registrar.uncommitted_events().len(), 4);
        let mut replayed = CurrencyRegistrar::new();
        for event in currency_registrar.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), currency_registrar.state());
    }
}
