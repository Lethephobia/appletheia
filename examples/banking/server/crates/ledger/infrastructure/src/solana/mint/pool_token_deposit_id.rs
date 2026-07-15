use appletheia::domain::AggregateId;
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PoolTokenDepositId([u8; 16]);

impl PoolTokenDepositId {
    pub(crate) fn into_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl From<DepositId> for PoolTokenDepositId {
    fn from(deposit_id: DepositId) -> Self {
        Self(*deposit_id.value().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use appletheia::domain::AggregateId;
    use banking_ledger_domain::deposit::DepositId;

    use super::PoolTokenDepositId;

    #[test]
    fn converts_deposit_id_to_same_bytes() {
        let deposit_id = DepositId::new();

        let pool_token_deposit_id = PoolTokenDepositId::from(deposit_id);

        assert_eq!(
            pool_token_deposit_id.into_bytes(),
            *deposit_id.value().as_bytes()
        );
    }
}
