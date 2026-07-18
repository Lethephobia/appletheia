/// Describes the outcome of validating a token account owner address.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenAccountOwnerAddressValidationResult {
    Valid,
    Invalid,
}
