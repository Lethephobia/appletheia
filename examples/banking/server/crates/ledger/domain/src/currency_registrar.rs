mod currency_registrar_create_rejection_reason;
mod currency_registrar_create_result;
mod currency_registrar_creation;
mod currency_registrar_description;
mod currency_registrar_description_error;
mod currency_registrar_display_name;
mod currency_registrar_display_name_error;
mod currency_registrar_error;
mod currency_registrar_event_payload;
mod currency_registrar_event_payload_error;
mod currency_registrar_handle;
mod currency_registrar_handle_change_rejection_reason;
mod currency_registrar_handle_change_result;
mod currency_registrar_handle_error;
mod currency_registrar_id;
mod currency_registrar_state;
mod currency_registrar_state_error;

pub use currency_registrar_create_rejection_reason::CurrencyRegistrarCreateRejectionReason;
pub use currency_registrar_create_result::CurrencyRegistrarCreateResult;
pub use currency_registrar_creation::CurrencyRegistrarCreation;
pub use currency_registrar_description::CurrencyRegistrarDescription;
pub use currency_registrar_description_error::CurrencyRegistrarDescriptionError;
pub use currency_registrar_display_name::CurrencyRegistrarDisplayName;
pub use currency_registrar_display_name_error::CurrencyRegistrarDisplayNameError;
pub use currency_registrar_error::CurrencyRegistrarError;
pub use currency_registrar_event_payload::CurrencyRegistrarEventPayload;
pub use currency_registrar_event_payload_error::CurrencyRegistrarEventPayloadError;
pub use currency_registrar_handle::CurrencyRegistrarHandle;
pub use currency_registrar_handle_change_rejection_reason::CurrencyRegistrarHandleChangeRejectionReason;
pub use currency_registrar_handle_change_result::CurrencyRegistrarHandleChangeResult;
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
        creation: CurrencyRegistrarCreation,
    ) -> Result<CurrencyRegistrarCreateResult, CurrencyRegistrarError> {
        if self.state().is_some() {
            return Err(CurrencyRegistrarError::AlreadyCreated);
        }

        let (handle, display_name, description) = creation.into_parts();
        self.append_event(CurrencyRegistrarEventPayload::Created {
            handle,
            display_name,
            description,
        })?;
        Ok(CurrencyRegistrarCreateResult::Created)
    }

    pub fn reject_create(
        &mut self,
        _creation: CurrencyRegistrarCreation,
        reason: CurrencyRegistrarCreateRejectionReason,
    ) -> Result<(), CurrencyRegistrarError> {
        Err(CurrencyRegistrarError::CreateRejected(reason))
    }

    pub fn change_handle(
        &mut self,
        handle: CurrencyRegistrarHandle,
    ) -> Result<CurrencyRegistrarHandleChangeResult, CurrencyRegistrarError> {
        self.state_required()?;
        self.append_event(CurrencyRegistrarEventPayload::HandleChanged { handle })?;
        Ok(CurrencyRegistrarHandleChangeResult::Changed)
    }

    pub fn reject_change_handle(
        &mut self,
        _handle: CurrencyRegistrarHandle,
        reason: CurrencyRegistrarHandleChangeRejectionReason,
    ) -> Result<CurrencyRegistrarHandleChangeResult, CurrencyRegistrarError> {
        Err(CurrencyRegistrarError::HandleChangeRejected(reason))
    }

    pub fn change_display_name(
        &mut self,
        display_name: CurrencyRegistrarDisplayName,
    ) -> Result<(), CurrencyRegistrarError> {
        self.state_required()?;
        self.append_event(CurrencyRegistrarEventPayload::DisplayNameChanged { display_name })?;
        Ok(())
    }

    pub fn change_description(
        &mut self,
        description: Option<CurrencyRegistrarDescription>,
    ) -> Result<(), CurrencyRegistrarError> {
        self.state_required()?;
        self.append_event(CurrencyRegistrarEventPayload::DescriptionChanged { description })?;
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
                description,
            } => {
                self.set_state(Some(CurrencyRegistrarState {
                    handle: handle.clone(),
                    display_name: display_name.clone(),
                    description: description.clone(),
                }));
            }
            CurrencyRegistrarEventPayload::HandleChanged { handle } => {
                self.state_required_mut()?.handle = handle.clone();
            }
            CurrencyRegistrarEventPayload::DisplayNameChanged { display_name } => {
                self.state_required_mut()?.display_name = display_name.clone();
            }
            CurrencyRegistrarEventPayload::DescriptionChanged { description } => {
                self.state_required_mut()?.description = description.clone();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::Aggregate;

    use super::{
        CurrencyRegistrar, CurrencyRegistrarCreation, CurrencyRegistrarDisplayName,
        CurrencyRegistrarHandle,
    };

    #[test]
    fn create_initializes_the_registrar() {
        let mut registrar = CurrencyRegistrar::new();

        registrar
            .create(CurrencyRegistrarCreation {
                handle: CurrencyRegistrarHandle::try_from("example")
                    .expect("handle should be valid"),
                display_name: CurrencyRegistrarDisplayName::try_from("Example")
                    .expect("display name should be valid"),
                description: None,
            })
            .expect("registrar should be created");

        assert!(registrar.state().is_some());
    }
}
