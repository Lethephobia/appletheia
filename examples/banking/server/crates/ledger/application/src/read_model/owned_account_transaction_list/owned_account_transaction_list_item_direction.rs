use serde::Serialize;

/// Direction of a transaction from the account's point of view.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum OwnedAccountTransactionListItemDirection {
    Incoming,
    Outgoing,
}
