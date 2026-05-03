use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload, TransferStatus};

use super::{TransferProjectorError, TransferProjectorSpec};
use crate::projection::{TransferProjectionStore, TransferProjectionUpsert};

/// Projects transfer events into normalized transfer projections.
pub struct TransferProjector<VS>
where
    VS: TransferProjectionStore,
{
    projection_store: VS,
}

impl<VS> TransferProjector<VS>
where
    VS: TransferProjectionStore,
{
    pub fn new(projection_store: VS) -> Self {
        Self { projection_store }
    }
}

impl<VS> Projector for TransferProjector<VS>
where
    VS: TransferProjectionStore,
{
    type Spec = TransferProjectorSpec;
    type Uow = VS::Uow;
    type Error = TransferProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let domain_event = event.try_into_domain_event::<Transfer>()?;
        let transfer_id = domain_event.aggregate_id();

        match domain_event.payload() {
            TransferEventPayload::Requested {
                from_account_id,
                to_account_id,
                amount,
                ..
            } => {
                self.projection_store
                    .upsert(
                        uow,
                        TransferProjectionUpsert {
                            id: transfer_id,
                            from_account_id: *from_account_id,
                            to_account_id: *to_account_id,
                            amount: *amount,
                            status: TransferStatus::Pending,
                        },
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            TransferEventPayload::Completed => {
                self.projection_store
                    .update_status(
                        uow,
                        transfer_id,
                        TransferStatus::Completed,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            TransferEventPayload::Failed { .. } => {
                self.projection_store
                    .update_status(
                        uow,
                        transfer_id,
                        TransferStatus::Failed,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            TransferEventPayload::Cancelled => {
                self.projection_store
                    .update_status(
                        uow,
                        transfer_id,
                        TransferStatus::Cancelled,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            TransferEventPayload::RequestRejected { .. }
            | TransferEventPayload::CompleteRejected { .. }
            | TransferEventPayload::FailRejected { .. }
            | TransferEventPayload::CancelRejected { .. } => {}
        }

        Ok(())
    }
}
