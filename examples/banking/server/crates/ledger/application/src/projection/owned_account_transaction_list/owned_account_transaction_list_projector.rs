use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};
use banking_ledger_domain::transfer::{Transfer, TransferEventPayload};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

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
        let event_context = ReadModelEventContext::from(event);
        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_owner_user(
                            uow,
                            event_context,
                            OwnedAccountTransactionListOwnerUserUpsert {
                                id: user_id,
                                username: None,
                                display_name: None,
                                picture: None,
                            },
                        )
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_user_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_user_display_name(
                            uow,
                            event_context,
                            user_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_user_picture(uow, event_context, user_id, picture.clone())
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
                | UserEventPayload::Deactivated
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. }
                | UserEventPayload::Removed
                | UserEventPayload::OrganizationMembershipGranted { .. }
                | UserEventPayload::OrganizationMembershipGrantRejected { .. }
                | UserEventPayload::OrganizationMembershipRolesChanged { .. }
                | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
                | UserEventPayload::OrganizationMembershipRemoved { .. }
                | UserEventPayload::OrganizationMembershipRemoveRejected { .. } => {}
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
                            event_context,
                            OwnedAccountTransactionListOwnerOrganizationUpsert {
                                id: organization_id,
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_owner_organization_handle(
                            uow,
                            event_context,
                            organization_id,
                            handle.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_organization_display_name(
                            uow,
                            event_context,
                            organization_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_organization_picture(
                            uow,
                            event_context,
                            organization_id,
                            picture.clone(),
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
                            event_context,
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
                            },
                        )
                        .await?;
                }
                AccountEventPayload::Withdrawn { amount } => {
                    self.writer
                        .insert_account_transaction(
                            uow,
                            event_context,
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
                            event_context,
                            OwnedAccountTransactionListCurrencyUpsert {
                                id: currency_id,
                                symbol: symbol.clone(),
                                name: name.clone(),
                                decimals: *decimals,
                                mint_account_address: None,
                            },
                        )
                        .await?;
                }
                CurrencyEventPayload::Provisioned { mint_account } => {
                    self.writer
                        .update_mint_account_address(
                            uow,
                            event_context,
                            currency_id,
                            mint_account.mint_account_address().clone(),
                        )
                        .await?;
                }
                CurrencyEventPayload::SymbolChanged { symbol } => {
                    self.writer
                        .update_currency_symbol(uow, event_context, currency_id, symbol.clone())
                        .await?;
                }
                CurrencyEventPayload::NameChanged { name } => {
                    self.writer
                        .update_currency_name(uow, event_context, currency_id, name.clone())
                        .await?;
                }
                CurrencyEventPayload::Removed => {
                    self.writer
                        .delete_currency(uow, event_context, currency_id)
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
                            event_context,
                            OwnedAccountTransactionListTransferRequestedRecord {
                                id: transfer_id,
                                correlation_id: event.correlation_id,
                                from_account_id: *from_account_id,
                                to_account_id: *to_account_id,
                                amount: *amount,
                            },
                        )
                        .await?;
                }
                TransferEventPayload::Completed => {
                    self.writer
                        .complete_transfer(
                            uow,
                            event_context,
                            transfer_id,
                            OwnedAccountTransactionId::from(event.event_id.value()),
                        )
                        .await?;
                }
                TransferEventPayload::Failed { reason } => {
                    self.writer
                        .fail_transfer(uow, event_context, transfer_id, *reason)
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
                        event_context,
                        OwnedAccountTransactionListCurrencyIssuanceIssuedRecord {
                            id: currency_issuance_id,
                            destination_account_id: *destination_account_id,
                            currency_id: *currency_id,
                            amount: *amount,
                        },
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Completed => {
                self.writer
                    .complete_currency_issuance(
                        uow,
                        event_context,
                        currency_issuance_id,
                        OwnedAccountTransactionId::from(event.event_id.value()),
                    )
                    .await?;
            }
            CurrencyIssuanceEventPayload::Failed { .. } => {
                self.writer
                    .fail_currency_issuance(uow, event_context, currency_issuance_id)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
