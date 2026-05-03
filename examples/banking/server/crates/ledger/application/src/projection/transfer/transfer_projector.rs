use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload, TransferStatus};

use super::{TransferProjectorError, TransferProjectorSpec};
use crate::view::{TransferViewStore, TransferViewUpsert};

/// Projects transfer events into normalized transfer views.
pub struct TransferProjector<VS>
where
    VS: TransferViewStore,
{
    view_store: VS,
}

impl<VS> TransferProjector<VS>
where
    VS: TransferViewStore,
{
    pub fn new(view_store: VS) -> Self {
        Self { view_store }
    }
}

impl<VS> Projector for TransferProjector<VS>
where
    VS: TransferViewStore,
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
                self.view_store
                    .upsert(
                        uow,
                        TransferViewUpsert {
                            id: transfer_id,
                            from_account_id: *from_account_id,
                            to_account_id: *to_account_id,
                            amount: *amount,
                            status: TransferStatus::Pending,
                        },
                        event.event_sequence,
                    )
                    .await?;
            }
            TransferEventPayload::Completed => {
                self.view_store
                    .update_status(
                        uow,
                        transfer_id,
                        TransferStatus::Completed,
                        event.event_sequence,
                    )
                    .await?;
            }
            TransferEventPayload::Failed { .. } => {
                self.view_store
                    .update_status(
                        uow,
                        transfer_id,
                        TransferStatus::Failed,
                        event.event_sequence,
                    )
                    .await?;
            }
            TransferEventPayload::Cancelled => {
                self.view_store
                    .update_status(
                        uow,
                        transfer_id,
                        TransferStatus::Cancelled,
                        event.event_sequence,
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
