use appletheia::application::read_model::{ReadModelObservation, ReadModelObservationSource};
use banking_iam_application::{OrganizationFragment, UserFragment};
use banking_ledger_domain::account::AccountOwner;
use banking_ledger_domain::currency::CurrencyOwner;
use banking_ledger_domain::wallet_bookmark::WalletBookmarkOwner;
use serde::{Deserialize, Serialize};

/// Fully materialized user or organization that owns a Ledger fragment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FragmentOwner {
    User(Box<UserFragment>),
    Organization(Box<OrganizationFragment>),
}

impl FragmentOwner {
    pub fn account_owner(&self) -> AccountOwner {
        match self {
            Self::User(user) => AccountOwner::User(user.id),
            Self::Organization(organization) => AccountOwner::Organization(organization.id),
        }
    }

    pub fn currency_owner(&self) -> CurrencyOwner {
        match self {
            Self::User(user) => CurrencyOwner::User(user.id),
            Self::Organization(organization) => CurrencyOwner::Organization(organization.id),
        }
    }

    pub fn wallet_bookmark_owner(&self) -> WalletBookmarkOwner {
        match self {
            Self::User(user) => WalletBookmarkOwner::User(user.id),
            Self::Organization(organization) => WalletBookmarkOwner::Organization(organization.id),
        }
    }
}

impl ReadModelObservationSource for FragmentOwner {
    fn observations(&self) -> Vec<ReadModelObservation> {
        match self {
            Self::User(user) => user.observations(),
            Self::Organization(organization) => organization.observations(),
        }
    }
}
