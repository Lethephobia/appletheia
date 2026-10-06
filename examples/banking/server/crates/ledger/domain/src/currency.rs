mod currency_description;
mod currency_description_error;
mod currency_error;
mod currency_event_payload;
mod currency_event_payload_error;
mod currency_id;
mod currency_state;
mod currency_state_error;
mod currency_status;

pub use currency_description::CurrencyDescription;
pub use currency_description_error::CurrencyDescriptionError;
pub use currency_error::CurrencyError;
pub use currency_event_payload::CurrencyEventPayload;
pub use currency_event_payload_error::CurrencyEventPayloadError;
pub use currency_id::CurrencyId;
pub use currency_state::CurrencyState;
pub use currency_state_error::CurrencyStateError;
pub use currency_status::CurrencyStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::core::{CurrencyCode, CurrencyDecimals};
use crate::currency_registrar::CurrencyRegistrarId;

/// Represents a registered currency.
#[aggregate(type = "currency", error = CurrencyError)]
pub struct Currency {
    core: AggregateCore<CurrencyId, CurrencyState, CurrencyEventPayload>,
}

impl Currency {
    pub fn currency_registrar_id(&self) -> Result<&CurrencyRegistrarId, CurrencyError> {
        Ok(&self.state_required()?.currency_registrar_id)
    }

    pub fn code(&self) -> Result<&CurrencyCode, CurrencyError> {
        Ok(&self.state_required()?.code)
    }

    pub fn decimals(&self) -> Result<CurrencyDecimals, CurrencyError> {
        Ok(self.state_required()?.decimals)
    }

    pub fn description(&self) -> Result<Option<&CurrencyDescription>, CurrencyError> {
        Ok(self.state_required()?.description.as_ref())
    }

    pub fn status(&self) -> Result<CurrencyStatus, CurrencyError> {
        Ok(self.state_required()?.status)
    }

    pub fn is_active(&self) -> Result<bool, CurrencyError> {
        Ok(self.state_required()?.status.is_active())
    }

    pub fn define(
        &mut self,
        currency_registrar_id: CurrencyRegistrarId,
        code: CurrencyCode,
        decimals: CurrencyDecimals,
    ) -> Result<(), CurrencyError> {
        if self.state().is_some() {
            return Err(CurrencyError::AlreadyDefined);
        }
        self.append_event(CurrencyEventPayload::Defined {
            currency_registrar_id,
            code,
            decimals,
        })?;
        Ok(())
    }

    pub fn set_description(
        &mut self,
        description: Option<CurrencyDescription>,
    ) -> Result<(), CurrencyError> {
        self.state_required()?;
        self.append_event(CurrencyEventPayload::DescriptionSet { description })?;
        Ok(())
    }

    pub fn activate(&mut self) -> Result<(), CurrencyError> {
        if self.state_required()?.status.is_active() {
            return Err(CurrencyError::AlreadyActive);
        }
        self.append_event(CurrencyEventPayload::Activated)?;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), CurrencyError> {
        if !self.state_required()?.status.is_active() {
            return Err(CurrencyError::AlreadyInactive);
        }
        self.append_event(CurrencyEventPayload::Deactivated)?;
        Ok(())
    }
}

impl AggregateApply<CurrencyEventPayload, CurrencyError> for Currency {
    fn apply(&mut self, payload: &CurrencyEventPayload) -> Result<(), CurrencyError> {
        match payload {
            CurrencyEventPayload::Defined {
                currency_registrar_id,
                code,
                decimals,
            } => self.set_state(Some(CurrencyState {
                currency_registrar_id: *currency_registrar_id,
                code: code.clone(),
                decimals: *decimals,
                description: None,
                status: CurrencyStatus::Defined,
            })),
            CurrencyEventPayload::DescriptionSet { description } => {
                self.state_required_mut()?.description = description.clone();
            }
            CurrencyEventPayload::Activated => {
                self.state_required_mut()?.status = CurrencyStatus::Active;
            }
            CurrencyEventPayload::Deactivated => {
                self.state_required_mut()?.status = CurrencyStatus::Inactive;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, AggregateApply};

    use crate::core::{CurrencyCode, CurrencyDecimals};
    use crate::currency_registrar::CurrencyRegistrarId;

    use super::{Currency, CurrencyDescription};

    fn defined_currency() -> Currency {
        let mut currency = Currency::new();
        currency
            .define(
                CurrencyRegistrarId::new(),
                CurrencyCode::try_from("USD").expect("currency code should be valid"),
                CurrencyDecimals::new(2),
            )
            .expect("definition should succeed");
        currency
    }

    #[test]
    fn definition_records_the_responsible_registrar() {
        assert!(defined_currency().currency_registrar_id().is_ok());
    }

    #[test]
    fn repeated_lifecycle_commands_are_rejected() {
        let mut currency = defined_currency();
        currency
            .deactivate()
            .expect_err("repeated deactivation should fail");
    }

    #[test]
    fn description_changes_are_recorded_even_when_the_value_is_unchanged() {
        let mut currency = defined_currency();
        let description = CurrencyDescription::try_from("United States dollar")
            .expect("currency description should be valid");

        currency
            .set_description(Some(description.clone()))
            .expect("description change should succeed");
        currency
            .set_description(Some(description.clone()))
            .expect("repeated description change should succeed");

        assert_eq!(
            currency.description().expect("currency state should exist"),
            Some(&description)
        );
        assert_eq!(currency.uncommitted_events().len(), 3);
    }

    #[test]
    fn optional_fields_are_set_separately_and_replayable() {
        let mut currency = Currency::new();
        currency
            .define(
                CurrencyRegistrarId::new(),
                CurrencyCode::try_from("USD").unwrap(),
                CurrencyDecimals::new(2),
            )
            .unwrap();
        assert_eq!(currency.uncommitted_events().len(), 1);
        assert_eq!(currency.description().unwrap(), None);
        let value = CurrencyDescription::try_from("Dollar").unwrap();
        currency.set_description(Some(value.clone())).unwrap();
        assert_eq!(currency.description().unwrap(), Some(&value));
        currency.set_description(None).unwrap();
        currency.set_description(None).unwrap();
        assert_eq!(currency.description().unwrap(), None);
        assert_eq!(currency.uncommitted_events().len(), 4);
        let mut replayed = Currency::new();
        for event in currency.uncommitted_events() {
            replayed.apply(event.payload()).unwrap();
        }
        assert_eq!(replayed.state(), currency.state());
    }
}
