use serde::{Deserialize, Serialize};

use super::{
    Erc3009ReceiveAuthorizationNonce, Erc3009ReceiveAuthorizationSignature,
    Erc3009ReceiveAuthorizationValidAfter, Erc3009ReceiveAuthorizationValidBefore,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Erc3009ReceiveAuthorization {
    valid_after: Erc3009ReceiveAuthorizationValidAfter,
    valid_before: Erc3009ReceiveAuthorizationValidBefore,
    nonce: Erc3009ReceiveAuthorizationNonce,
    signature: Erc3009ReceiveAuthorizationSignature,
}

impl Erc3009ReceiveAuthorization {
    pub const fn new(
        valid_after: Erc3009ReceiveAuthorizationValidAfter,
        valid_before: Erc3009ReceiveAuthorizationValidBefore,
        nonce: Erc3009ReceiveAuthorizationNonce,
        signature: Erc3009ReceiveAuthorizationSignature,
    ) -> Self {
        Self {
            valid_after,
            valid_before,
            nonce,
            signature,
        }
    }

    pub const fn valid_after(&self) -> Erc3009ReceiveAuthorizationValidAfter {
        self.valid_after
    }

    pub const fn valid_before(&self) -> Erc3009ReceiveAuthorizationValidBefore {
        self.valid_before
    }

    pub const fn nonce(&self) -> Erc3009ReceiveAuthorizationNonce {
        self.nonce
    }

    pub const fn signature(&self) -> Erc3009ReceiveAuthorizationSignature {
        self.signature
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Erc3009ReceiveAuthorization, Erc3009ReceiveAuthorizationNonce,
        Erc3009ReceiveAuthorizationSignature, Erc3009ReceiveAuthorizationValidAfter,
        Erc3009ReceiveAuthorizationValidBefore,
    };

    #[test]
    fn stores_authorization_bytes_and_serializes_as_prefixed_hexadecimal() {
        let authorization = Erc3009ReceiveAuthorization::new(
            Erc3009ReceiveAuthorizationValidAfter::new(10),
            Erc3009ReceiveAuthorizationValidBefore::new(20),
            Erc3009ReceiveAuthorizationNonce::from_bytes([0xcd; 32]),
            Erc3009ReceiveAuthorizationSignature::from_bytes([0xab; 65]),
        );
        let encoded = format!(
            "{{\"valid_after\":10,\"valid_before\":20,\"nonce\":\"0x{}\",\"signature\":\"0x{}\"}}",
            "cd".repeat(32),
            "ab".repeat(65)
        );

        assert_eq!(authorization.valid_after().value(), 10);
        assert_eq!(authorization.valid_before().value(), 20);
        assert_eq!(authorization.nonce().as_bytes(), &[0xcd; 32]);
        assert_eq!(authorization.signature().as_bytes(), &[0xab; 65]);
        assert_eq!(
            serde_json::to_string(&authorization).expect("authorization should serialize"),
            encoded
        );
        assert_eq!(
            serde_json::from_str::<Erc3009ReceiveAuthorization>(&encoded)
                .expect("authorization should deserialize"),
            authorization
        );
    }

    #[test]
    fn rejects_invalid_nonce_and_signatures() {
        assert!(
            serde_json::from_str::<Erc3009ReceiveAuthorization>(
                "{\"valid_after\":10,\"valid_before\":20,\"nonce\":\"0x12\",\"signature\":\"0x12\"}"
            )
            .is_err()
        );
    }
}
