use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

use super::{
    OwnedAccountTransactionListItemProjectorError, OwnedAccountTransactionListItemProjectorSpec,
};
use crate::read_model::{
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus, OwnedAccountTransactionListItemWriter,
};

/// Projects ledger events into owned account transaction list item read models.
pub struct OwnedAccountTransactionListItemProjector<W>
where
    W: OwnedAccountTransactionListItemWriter,
{
    writer: W,
}

impl<W> OwnedAccountTransactionListItemProjector<W>
where
    W: OwnedAccountTransactionListItemWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OwnedAccountTransactionListItemProjector<W>
where
    W: OwnedAccountTransactionListItemWriter,
{
    type Spec = OwnedAccountTransactionListItemProjectorSpec;
    type Uow = W::Uow;
    type Error = OwnedAccountTransactionListItemProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_owner_user(uow, user_id, event.event_sequence, event.occurred_at)
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_user_username(
                            uow,
                            user_id,
                            username.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_user_display_name(
                            uow,
                            user_id,
                            display_name.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_user_picture(
                            uow,
                            user_id,
                            picture.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::IdentityLinked { .. }
                | UserEventPayload::IdentityEmailChanged { .. }
                | UserEventPayload::BioChanged { .. }
                | UserEventPayload::Activated
                | UserEventPayload::Inactivated
                | UserEventPayload::Removed => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Organization>() {
            let domain_event = event.try_into_domain_event::<Organization>()?;
            let organization_id = domain_event.aggregate_id();

            match domain_event.payload() {
                OrganizationEventPayload::Created {
                    handle,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_owner_organization(
                            uow,
                            organization_id,
                            handle.clone(),
                            display_name.clone(),
                            picture.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_owner_organization_handle(
                            uow,
                            organization_id,
                            handle.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_organization_display_name(
                            uow,
                            organization_id,
                            display_name.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_organization_picture(
                            uow,
                            organization_id,
                            picture.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::OwnershipTransferred { .. }
                | OrganizationEventPayload::OwnershipTransferRejected { .. }
                | OrganizationEventPayload::HandleChangeRejected { .. }
                | OrganizationEventPayload::DisplayNameChangeRejected { .. }
                | OrganizationEventPayload::DescriptionChanged { .. }
                | OrganizationEventPayload::DescriptionChangeRejected { .. }
                | OrganizationEventPayload::WebsiteUrlChanged { .. }
                | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
                | OrganizationEventPayload::PictureChangeRejected { .. }
                | OrganizationEventPayload::RemoveRejected { .. }
                | OrganizationEventPayload::Removed => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Account>() {
            let domain_event = event.try_into_domain_event::<Account>()?;
            let account_id = domain_event.aggregate_id();

            match domain_event.payload() {
                AccountEventPayload::Deposited { amount } => {
                    self.writer
                        .insert_account_transaction(
                            uow,
                            event.event_id,
                            event.correlation_id,
                            account_id,
                            None,
                            *amount,
                            OwnedAccountTransactionListItemDirection::Incoming,
                            OwnedAccountTransactionListItemKind::Deposit,
                            OwnedAccountTransactionListItemStatus::Completed,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Withdrawn { amount } => {
                    self.writer
                        .insert_account_transaction(
                            uow,
                            event.event_id,
                            event.correlation_id,
                            account_id,
                            None,
                            *amount,
                            OwnedAccountTransactionListItemDirection::Outgoing,
                            OwnedAccountTransactionListItemKind::Withdrawal,
                            OwnedAccountTransactionListItemStatus::Completed,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                _ => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Currency>() {
            let domain_event = event.try_into_domain_event::<Currency>()?;
            let currency_id = domain_event.aggregate_id();

            match domain_event.payload() {
                CurrencyEventPayload::Defined {
                    symbol,
                    name,
                    decimals,
                    ..
                } => {
                    self.writer
                        .upsert_currency(
                            uow,
                            currency_id,
                            symbol.clone(),
                            name.clone(),
                            *decimals,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                CurrencyEventPayload::SymbolChanged { symbol } => {
                    self.writer
                        .update_currency_symbol(
                            uow,
                            currency_id,
                            symbol.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                CurrencyEventPayload::NameChanged { name } => {
                    self.writer
                        .update_currency_name(
                            uow,
                            currency_id,
                            name.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                CurrencyEventPayload::Removed => {
                    self.writer
                        .delete_currency(uow, currency_id, event.event_sequence)
                        .await?;
                }
                _ => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Transfer>() {
            let domain_event = event.try_into_domain_event::<Transfer>()?;
            let transfer_id = domain_event.aggregate_id();

            match domain_event.payload() {
                TransferEventPayload::Requested {
                    from_account_id,
                    to_account_id,
                    amount,
                    ..
                } => {
                    self.writer
                        .record_transfer_requested(
                            uow,
                            transfer_id,
                            event.correlation_id,
                            *from_account_id,
                            *to_account_id,
                            *amount,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                TransferEventPayload::Completed => {
                    self.writer
                        .complete_transfer(
                            uow,
                            transfer_id,
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                TransferEventPayload::Failed { reason } => {
                    self.writer
                        .fail_transfer(
                            uow,
                            transfer_id,
                            *reason,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                _ => {}
            }

            return Ok(());
        }

        let domain_event = event.try_into_domain_event::<CurrencyIssuance>()?;
        let currency_issuance_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyIssuanceEventPayload::Issued {
                destination_account_id,
                currency_id,
                amount,
                ..
            } => {
                self.writer
                    .record_currency_issuance_issued(
                        uow,
                        currency_issuance_id,
                        *destination_account_id,
                        *currency_id,
                        *amount,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Completed => {
                self.writer
                    .complete_currency_issuance(
                        uow,
                        currency_issuance_id,
                        event.event_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Failed { .. } => {
                self.writer
                    .fail_currency_issuance(uow, currency_issuance_id, event.event_sequence)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
