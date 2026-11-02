use serde::{Deserialize, Serialize};

use super::{Erc2612PermitDeadline, Erc2612PermitSignature};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Erc2612Permit {
    deadline: Erc2612PermitDeadline,
    signature: Erc2612PermitSignature,
}

impl Erc2612Permit {
    pub const fn new(deadline: Erc2612PermitDeadline, signature: Erc2612PermitSignature) -> Self {
        Self {
            deadline,
            signature,
        }
    }

    pub const fn deadline(&self) -> Erc2612PermitDeadline {
        self.deadline
    }

    pub const fn signature(&self) -> Erc2612PermitSignature {
        self.signature
    }
}

#[cfg(test)]
mod tests {
    use super::{Erc2612Permit, Erc2612PermitDeadline, Erc2612PermitSignature};

    #[test]
    fn stores_signature_bytes_and_serializes_as_prefixed_hexadecimal() {
        let permit = Erc2612Permit::new(
            Erc2612PermitDeadline::new(123),
            Erc2612PermitSignature::from_bytes([0xab; 65]),
        );
        let encoded = format!(
            "{{\"deadline\":123,\"signature\":\"0x{}\"}}",
            "ab".repeat(65)
        );

        assert_eq!(permit.deadline().value(), 123);
        assert_eq!(permit.signature().as_bytes(), &[0xab; 65]);
        assert_eq!(
            serde_json::to_string(&permit).expect("permit should serialize"),
            encoded
        );
        assert_eq!(
            serde_json::from_str::<Erc2612Permit>(&encoded).expect("permit should deserialize"),
            permit
        );
    }

    #[test]
    fn rejects_invalid_signatures() {
        assert!(
            serde_json::from_str::<Erc2612Permit>("{\"deadline\":123,\"signature\":\"0x12\"}")
                .is_err()
        );
    }
}
