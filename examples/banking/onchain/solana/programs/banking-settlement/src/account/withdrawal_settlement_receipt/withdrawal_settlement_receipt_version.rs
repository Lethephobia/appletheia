use anchor_lang::prelude::*;

/// Persisted variant indices: 0 is uninitialized, 1 is V1.
/// Preserve this order and append future versions to keep existing account data readable.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WithdrawalSettlementReceiptVersion {
    Uninitialized,
    V1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_existing_version_bytes() {
        for (version, byte) in [
            (WithdrawalSettlementReceiptVersion::Uninitialized, 0),
            (WithdrawalSettlementReceiptVersion::V1, 1),
        ] {
            let mut encoded = Vec::new();
            version.serialize(&mut encoded).unwrap();
            assert_eq!(encoded, vec![byte]);
            assert_eq!(
                WithdrawalSettlementReceiptVersion::try_from_slice(&[byte]).unwrap(),
                version
            );
        }
    }

    #[test]
    fn rejects_unknown_versions() {
        for byte in 2..=u8::MAX {
            assert!(WithdrawalSettlementReceiptVersion::try_from_slice(&[byte]).is_err());
        }
    }
}
