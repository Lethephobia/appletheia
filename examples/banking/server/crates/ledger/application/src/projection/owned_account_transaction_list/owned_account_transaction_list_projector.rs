use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};

use super::{OwnedAccountTransactionListProjectorError, OwnedAccountTransactionListProjectorSpec};
use crate::read_model::{
    OwnedAccountTransactionId, OwnedAccountTransactionListCurrencyIssuanceIssuedRecord,
    OwnedAccountTransactionListCurrencyUpsert, OwnedAccountTransactionListItemDirection,
    OwnedAccountTransactionListItemInsert, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus, OwnedAccountTransactionListOwnerOrganizationUpsert,
    OwnedAccountTransactionListOwnerUserUpsert, OwnedAccountTransactionListTransferRequestedRecord,
    OwnedAccountTransactionListWriter,
};

/// Projects ledger events into owned account transaction list read models.
pub struct OwnedAccountTransactionListProjector<W>
where
    W: OwnedAccountTransactionListWriter,
{
    writer: W,
}

impl<W> OwnedAccountTransactionListProjector<W>
where
    W: OwnedAccountTransactionListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OwnedAccountTransactionListProjector<W>
where
    W: OwnedAccountTransactionListWriter,
{
    type Spec = OwnedAccountTransactionListProjectorSpec;
    type Uow = W::Uow;
    type Error = OwnedAccountTransactionListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered {
                    username,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_owner_user(
                            uow,
                            OwnedAccountTransactionListOwnerUserUpsert {
                                id: user_id,
                                username: username.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_user_username(
                            uow,
                            user_id,
                            username.clone(),
                            event.event_id,
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
                            event.event_id,
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
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::IdentityLinked { .. }
                | UserEventPayload::IdentityLinkRejected { .. }
                | UserEventPayload::IdentityEmailChanged { .. }
                | UserEventPayload::IdentityEmailChangeRejected { .. }
                | UserEventPayload::BioChanged { .. }
                | UserEventPayload::BioChangeRejected { .. }
                | UserEventPayload::UsernameChangeRejected { .. }
                | UserEventPayload::DisplayNameChangeRejected { .. }
                | UserEventPayload::PictureChangeRejected { .. }
                | UserEventPayload::Activated
                | UserEventPayload::ActivateRejected { .. }
                | UserEventPayload::Inactivated
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. }
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
                            OwnedAccountTransactionListOwnerOrganizationUpsert {
                                id: organization_id,
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_owner_organization_handle(
                            uow,
                            organization_id,
                            handle.clone(),
                            event.event_id,
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
                            event.event_id,
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
                            event.event_id,
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
                            OwnedAccountTransactionListItemInsert {
                                transaction_id: OwnedAccountTransactionId::from(
                                    event.event_id.value(),
                                ),
                                correlation_id: event.correlation_id,
                                account_id,
                                counterparty_account_id: None,
                                amount: *amount,
                                direction: OwnedAccountTransactionListItemDirection::Incoming,
                                kind: OwnedAccountTransactionListItemKind::Deposit,
                                status: OwnedAccountTransactionListItemStatus::Completed,
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                AccountEventPayload::Withdrawn { amount } => {
                    self.writer
                        .insert_account_transaction(
                            uow,
                            OwnedAccountTransactionListItemInsert {
                                transaction_id: OwnedAccountTransactionId::from(
                                    event.event_id.value(),
                                ),
                                correlation_id: event.correlation_id,
                                account_id,
                                counterparty_account_id: None,
                                amount: *amount,
                                direction: OwnedAccountTransactionListItemDirection::Outgoing,
                                kind: OwnedAccountTransactionListItemKind::Withdrawal,
                                status: OwnedAccountTransactionListItemStatus::Completed,
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
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
                            OwnedAccountTransactionListCurrencyUpsert {
                                id: currency_id,
                                symbol: symbol.clone(),
                                name: name.clone(),
                                decimals: *decimals,
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                CurrencyEventPayload::SymbolChanged { symbol } => {
                    self.writer
                        .update_currency_symbol(
                            uow,
                            currency_id,
                            symbol.clone(),
                            event.event_id,
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
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                CurrencyEventPayload::Removed => {
                    self.writer
                        .delete_currency(uow, currency_id, event.event_id, event.event_sequence)
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
                            OwnedAccountTransactionListTransferRequestedRecord {
                                id: transfer_id,
                                correlation_id: event.correlation_id,
                                from_account_id: *from_account_id,
                                to_account_id: *to_account_id,
                                amount: *amount,
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                TransferEventPayload::Completed => {
                    self.writer
                        .complete_transfer(
                            uow,
                            transfer_id,
                            OwnedAccountTransactionId::from(event.event_id.value()),
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
                            event.event_id,
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
                        OwnedAccountTransactionListCurrencyIssuanceIssuedRecord {
                            id: currency_issuance_id,
                            destination_account_id: *destination_account_id,
                            currency_id: *currency_id,
                            amount: *amount,
                            event_id: event.event_id,
                            event_sequence: event.event_sequence,
                            occurred_at: event.occurred_at,
                        },
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Completed => {
                self.writer
                    .complete_currency_issuance(
                        uow,
                        currency_issuance_id,
                        OwnedAccountTransactionId::from(event.event_id.value()),
                        event.event_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Failed { .. } => {
                self.writer
                    .fail_currency_issuance(
                        uow,
                        currency_issuance_id,
                        event.event_id,
                        event.event_sequence,
                    )
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
