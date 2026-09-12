mod account;
mod currency;
mod currency_registrar_invitation;
mod currency_registrar_join_request;
mod currency_registrar_membership;
mod token_binding;
mod wallet_bookmark;

pub use banking_iam_application::authorization::UserOwnerRelation;

pub use account::*;
pub use currency::*;
pub use currency_registrar_invitation::*;
pub use currency_registrar_join_request::*;
pub use currency_registrar_membership::*;
pub use token_binding::*;
pub use wallet_bookmark::*;

/// Registers this application module's evaluation and derivation declarations.
pub fn define_ledger_relations(
    model: &mut appletheia::application::authorization::InMemoryAuthorizationModel,
) {
    model.define_relation(AccountCloserRelation);
    model.define_relation(AccountDepositRequesterRelation);
    model.define_relation(AccountDescriptionChangerRelation);
    model.define_relation(AccountFreezerRelation);
    model.define_relation(AccountNameChangerRelation);
    model.define_relation(AccountOwnerRelation);
    model.define_relation(AccountOwnershipTransfererRelation);
    model.define_relation(AccountStatusManagerRelation);
    model.define_relation(AccountThawerRelation);
    model.define_relation(AccountTransferRequesterRelation);
    model.define_relation(AccountWithdrawalRequesterRelation);
    model.define_relation(CurrencyActivatorRelation);
    model.define_relation(CurrencyDeactivatorRelation);
    model.define_relation(CurrencyDescriptionChangerRelation);
    model.define_relation(CurrencyManagerRelation);
    model.define_relation(CurrencyRegistrarCurrencyDefinerRelation);
    model.define_relation(CurrencyRegistrarRelation);
    model.define_relation(CurrencyTokenBindingDefinerRelation);
    model.define_relation(CurrencyRegistrarInvitationCancelerRelation);
    model.define_relation(CurrencyRegistrarInvitationInviteeRelation);
    model.define_relation(CurrencyRegistrarInvitationRegistrarRelation);
    model.define_relation(CurrencyRegistrarJoinRequestApproverRelation);
    model.define_relation(CurrencyRegistrarJoinRequestCancelerRelation);
    model.define_relation(CurrencyRegistrarJoinRequestRegistrarRelation);
    model.define_relation(CurrencyRegistrarJoinRequestRejecterRelation);
    model.define_relation(CurrencyRegistrarJoinRequestRequesterRelation);
    model.define_relation(CurrencyRegistrarMemberRelation);
    model.define_relation(CurrencyRegistrarMembershipRegistrarRelation);
    model.define_relation(CurrencyRegistrarMembershipRemoverRelation);
    model.define_relation(TokenBindingCurrencyRelation);
    model.define_relation(TokenBindingDepositEnabledChangerRelation);
    model.define_relation(TokenBindingRemoverRelation);
    model.define_relation(TokenBindingWithdrawalEnabledChangerRelation);
    model.define_relation(WalletBookmarkOwnerRelation);
    model.define_relation(WalletBookmarkRemoverRelation);
    model.define_relation(WalletBookmarkUpdaterRelation);
}
