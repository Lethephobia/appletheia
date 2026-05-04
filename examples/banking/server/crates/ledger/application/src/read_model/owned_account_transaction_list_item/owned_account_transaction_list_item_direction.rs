/// Direction of a transaction from the account's point of view.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OwnedAccountTransactionListItemDirection {
    Incoming,
    Outgoing,
}
