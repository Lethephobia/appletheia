use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::currency_issuance::{
    CurrencyIssuance, CurrencyIssuanceEventPayload, CurrencyIssuanceStatus,
};

use super::{CurrencyIssuanceProjectorError, CurrencyIssuanceProjectorSpec};
use crate::view::{CurrencyIssuanceViewStore, CurrencyIssuanceViewUpsert};

/// Projects currency issuance events into normalized currency issuance views.
pub struct CurrencyIssuanceProjector<VS>
where
    VS: CurrencyIssuanceViewStore,
{
    view_store: VS,
}

impl<VS> CurrencyIssuanceProjector<VS>
where
    VS: CurrencyIssuanceViewStore,
{
    pub fn new(view_store: VS) -> Self {
        Self { view_store }
    }
}

impl<VS> Projector for CurrencyIssuanceProjector<VS>
where
    VS: CurrencyIssuanceViewStore,
{
    type Spec = CurrencyIssuanceProjectorSpec;
    type Uow = VS::Uow;
    type Error = CurrencyIssuanceProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<CurrencyIssuance>()?;
        let issuance_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyIssuanceEventPayload::Issued {
                currency_id,
                destination_account_id,
                amount,
                ..
            } => {
                self.view_store
                    .upsert(
                        uow,
                        CurrencyIssuanceViewUpsert {
                            id: issuance_id,
                            currency_id: *currency_id,
                            destination_account_id: *destination_account_id,
                            amount: *amount,
                            status: CurrencyIssuanceStatus::Pending,
                        },
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Completed => {
                self.view_store
                    .update_status(
                        uow,
                        issuance_id,
                        CurrencyIssuanceStatus::Completed,
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Failed { .. } => {
                self.view_store
                    .update_status(
                        uow,
                        issuance_id,
                        CurrencyIssuanceStatus::Failed,
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::IssueRejected { .. }
            | CurrencyIssuanceEventPayload::CompleteRejected { .. }
            | CurrencyIssuanceEventPayload::FailRejected { .. } => {}
        }

        Ok(())
    }
}
