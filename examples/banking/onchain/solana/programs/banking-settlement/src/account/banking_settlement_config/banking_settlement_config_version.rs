use anchor_lang::prelude::*;

/// Persisted variant indices: 0 is uninitialized, 1 is V1.
/// Preserve this order and append future versions to keep existing account data readable.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BankingSettlementConfigVersion {
    Uninitialized,
    V1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_existing_version_bytes() {
        for (version, byte) in [
            (BankingSettlementConfigVersion::Uninitialized, 0),
            (BankingSettlementConfigVersion::V1, 1),
        ] {
            let mut encoded = Vec::new();
            version.serialize(&mut encoded).unwrap();
            assert_eq!(encoded, vec![byte]);
            assert_eq!(
                BankingSettlementConfigVersion::try_from_slice(&[byte]).unwrap(),
                version
            );
        }
    }

    #[test]
    fn rejects_unknown_versions() {
        for byte in 2..=u8::MAX {
            assert!(BankingSettlementConfigVersion::try_from_slice(&[byte]).is_err());
        }
    }
}
