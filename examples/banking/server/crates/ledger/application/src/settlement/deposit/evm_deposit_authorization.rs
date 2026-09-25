use serde::{Deserialize, Serialize};

use super::{Erc2612Permit, Erc3009ReceiveAuthorization};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum EvmDepositAuthorization {
    /// Sets an allowance with an ERC-2612 permit before settlement.
    Erc2612(Erc2612Permit),
    /// Transfers once with an ERC-3009 receive authorization.
    Erc3009(Erc3009ReceiveAuthorization),
}

#[cfg(test)]
mod tests {
    use crate::settlement::{
        Erc2612Permit, Erc2612PermitDeadline, Erc2612PermitSignature, Erc3009ReceiveAuthorization,
        Erc3009ReceiveAuthorizationNonce, Erc3009ReceiveAuthorizationSignature,
        Erc3009ReceiveAuthorizationValidAfter, Erc3009ReceiveAuthorizationValidBefore,
        EvmDepositAuthorization,
    };

    #[test]
    fn serializes_with_the_explicit_erc_standard() {
        let permit = EvmDepositAuthorization::Erc2612(Erc2612Permit::new(
            Erc2612PermitDeadline::new(123),
            Erc2612PermitSignature::from_bytes([0xab; 65]),
        ));
        let authorization = EvmDepositAuthorization::Erc3009(Erc3009ReceiveAuthorization::new(
            Erc3009ReceiveAuthorizationValidAfter::new(10),
            Erc3009ReceiveAuthorizationValidBefore::new(20),
            Erc3009ReceiveAuthorizationNonce::from_bytes([0xcd; 32]),
            Erc3009ReceiveAuthorizationSignature::from_bytes([0xef; 65]),
        ));

        let permit_json = serde_json::to_value(permit).expect("permit should serialize");
        let authorization_json =
            serde_json::to_value(authorization).expect("authorization should serialize");

        assert_eq!(permit_json["type"], "erc2612");
        assert_eq!(authorization_json["type"], "erc3009");
    }
}
