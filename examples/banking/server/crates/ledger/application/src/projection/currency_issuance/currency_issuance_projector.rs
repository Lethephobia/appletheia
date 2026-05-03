use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::currency_issuance::{
    CurrencyIssuance, CurrencyIssuanceEventPayload, CurrencyIssuanceStatus,
};

use super::{CurrencyIssuanceProjectorError, CurrencyIssuanceProjectorSpec};
use crate::projection::{CurrencyIssuanceProjectionStore, CurrencyIssuanceProjectionUpsert};

/// Projects currency issuance events into normalized currency issuance projections.
pub struct CurrencyIssuanceProjector<VS>
where
    VS: CurrencyIssuanceProjectionStore,
{
    projection_store: VS,
}

impl<VS> CurrencyIssuanceProjector<VS>
where
    VS: CurrencyIssuanceProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for CurrencyIssuanceProjector<VS>
where
    VS: CurrencyIssuanceProjectionStore,
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
                self.projection_store
                    .upsert(
                        uow,
                        CurrencyIssuanceProjectionUpsert {
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
                self.projection_store
                    .update_status(
                        uow,
                        issuance_id,
                        CurrencyIssuanceStatus::Completed,
                        event.event_sequence,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Failed { .. } => {
                self.projection_store
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
