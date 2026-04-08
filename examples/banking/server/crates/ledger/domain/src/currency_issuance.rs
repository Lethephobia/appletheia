mod currency_issuance_error;
mod currency_issuance_event_payload;
mod currency_issuance_event_payload_error;
mod currency_issuance_id;
mod currency_issuance_state;
mod currency_issuance_state_error;
mod currency_issuance_status;

pub use currency_issuance_error::CurrencyIssuanceError;
pub use currency_issuance_event_payload::CurrencyIssuanceEventPayload;
pub use currency_issuance_event_payload_error::CurrencyIssuanceEventPayloadError;
pub use currency_issuance_id::CurrencyIssuanceId;
pub use currency_issuance_state::CurrencyIssuanceState;
pub use currency_issuance_state_error::CurrencyIssuanceStateError;
pub use currency_issuance_status::CurrencyIssuanceStatus;

use appletheia::aggregate;
use appletheia::domain::{Aggregate, AggregateApply, AggregateCore};

use crate::account::AccountId;
use crate::core::CurrencyAmount;
use crate::currency_definition::CurrencyDefinitionId;

/// Represents the `CurrencyIssuance` aggregate root.
#[aggregate(type = "currency_issuance", error = CurrencyIssuanceError)]
pub struct CurrencyIssuance {
    core: AggregateCore<CurrencyIssuanceState, CurrencyIssuanceEventPayload>,
}

impl CurrencyIssuance {
    /// Returns the issued currency definition.
    pub fn currency_definition_id(&self) -> Result<&CurrencyDefinitionId, CurrencyIssuanceError> {
        Ok(&self.state_required()?.currency_definition_id)
    }

    /// Returns the destination account.
    pub fn destination_account_id(&self) -> Result<&AccountId, CurrencyIssuanceError> {
        Ok(&self.state_required()?.destination_account_id)
    }

    /// Returns the issuance amount.
    pub fn amount(&self) -> Result<&CurrencyAmount, CurrencyIssuanceError> {
        Ok(&self.state_required()?.amount)
    }

    /// Returns the current issuance status.
    pub fn status(&self) -> Result<&CurrencyIssuanceStatus, CurrencyIssuanceError> {
        Ok(&self.state_required()?.status)
    }

    /// Starts a new issuance workflow.
    pub fn issue(
        &mut self,
        currency_definition_id: CurrencyDefinitionId,
        destination_account_id: AccountId,
        amount: CurrencyAmount,
    ) -> Result<(), CurrencyIssuanceError> {
        if self.state().is_some() {
            return Err(CurrencyIssuanceError::AlreadyIssued);
        }

        if amount.is_zero() {
            return Err(CurrencyIssuanceError::ZeroAmount);
        }

        self.append_event(CurrencyIssuanceEventPayload::Issued {
            id: CurrencyIssuanceId::new(),
            currency_definition_id,
            destination_account_id,
            amount,
        })
    }

    /// Marks the issuance completed.
    pub fn complete(&mut self) -> Result<(), CurrencyIssuanceError> {
        self.ensure_pending()?;

        self.append_event(CurrencyIssuanceEventPayload::Completed)
    }

    /// Marks the issuance failed.
    pub fn fail(&mut self) -> Result<(), CurrencyIssuanceError> {
        self.ensure_pending()?;

        self.append_event(CurrencyIssuanceEventPayload::Failed)
    }

    fn ensure_pending(&self) -> Result<(), CurrencyIssuanceError> {
        match self.state_required()?.status {
            CurrencyIssuanceStatus::Pending => Ok(()),
            CurrencyIssuanceStatus::Completed => Err(CurrencyIssuanceError::AlreadyCompleted),
            CurrencyIssuanceStatus::Failed => Err(CurrencyIssuanceError::AlreadyFailed),
        }
    }
}

impl AggregateApply<CurrencyIssuanceEventPayload, CurrencyIssuanceError> for CurrencyIssuance {
    fn apply(
        &mut self,
        payload: &CurrencyIssuanceEventPayload,
    ) -> Result<(), CurrencyIssuanceError> {
        match payload {
            CurrencyIssuanceEventPayload::Issued {
                id,
                currency_definition_id,
                destination_account_id,
                amount,
            } => self.set_state(Some(CurrencyIssuanceState::new(
                *id,
                *currency_definition_id,
                *destination_account_id,
                *amount,
            ))),
            CurrencyIssuanceEventPayload::Completed => {
                self.state_required_mut()?.status = CurrencyIssuanceStatus::Completed;
            }
            CurrencyIssuanceEventPayload::Failed => {
                self.state_required_mut()?.status = CurrencyIssuanceStatus::Failed;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::{Aggregate, Event, EventPayload};

    use crate::account::AccountId;
    use crate::core::CurrencyAmount;
    use crate::currency_definition::CurrencyDefinitionId;

    use super::{
        CurrencyIssuance, CurrencyIssuanceEventPayload, CurrencyIssuanceId, CurrencyIssuanceStatus,
    };

    #[test]
    fn issue_initializes_state_and_records_event() {
        let currency_definition_id = CurrencyDefinitionId::new();
        let destination_account_id = AccountId::new();
        let amount = CurrencyAmount::new(100);
        let mut issuance = CurrencyIssuance::default();

        issuance
            .issue(currency_definition_id, destination_account_id, amount)
            .expect("issue should succeed");

        assert_eq!(
            issuance
                .currency_definition_id()
                .expect("currency definition id should exist"),
            &currency_definition_id
        );
        assert_eq!(
            issuance
                .destination_account_id()
                .expect("account id should exist"),
            &destination_account_id
        );
        assert_eq!(issuance.amount().expect("amount should exist"), &amount);
        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Pending
        );
        assert_eq!(
            issuance.uncommitted_events()[0].payload().name(),
            CurrencyIssuanceEventPayload::ISSUED
        );
    }

    #[test]
    fn issue_rejects_zero_amount() {
        let mut issuance = CurrencyIssuance::default();

        let error = issuance
            .issue(
                CurrencyDefinitionId::new(),
                AccountId::new(),
                CurrencyAmount::zero(),
            )
            .expect_err("zero amount should fail");

        assert!(matches!(error, super::CurrencyIssuanceError::ZeroAmount));
    }

    #[test]
    fn complete_updates_status() {
        let mut issuance = CurrencyIssuance::default();
        issuance
            .issue(
                CurrencyDefinitionId::new(),
                AccountId::new(),
                CurrencyAmount::new(100),
            )
            .expect("issue should succeed");

        issuance.complete().expect("complete should succeed");

        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Completed
        );
    }

    #[test]
    fn fail_updates_status() {
        let mut issuance = CurrencyIssuance::default();
        issuance
            .issue(
                CurrencyDefinitionId::new(),
                AccountId::new(),
                CurrencyAmount::new(100),
            )
            .expect("issue should succeed");

        issuance.fail().expect("fail should succeed");

        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Failed
        );
    }

    #[test]
    fn replay_events_rebuilds_state() {
        let id = CurrencyIssuanceId::new();
        let currency_definition_id = CurrencyDefinitionId::new();
        let destination_account_id = AccountId::new();
        let issued = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(1).expect("version should be valid"),
            CurrencyIssuanceEventPayload::Issued {
                id,
                currency_definition_id,
                destination_account_id,
                amount: CurrencyAmount::new(100),
            },
        );
        let completed = Event::new(
            id,
            appletheia::domain::AggregateVersion::try_from(2).expect("version should be valid"),
            CurrencyIssuanceEventPayload::Completed,
        );
        let mut issuance = CurrencyIssuance::default();

        issuance
            .replay_events(vec![issued, completed], None)
            .expect("events should replay");

        assert_eq!(
            issuance
                .currency_definition_id()
                .expect("currency definition id should exist"),
            &currency_definition_id
        );
        assert_eq!(
            issuance.status().expect("status should exist"),
            &CurrencyIssuanceStatus::Completed
        );
        assert_eq!(issuance.version().value(), 2);
        assert!(issuance.uncommitted_events().is_empty());
    }
}
