use banking_iam_domain::UserId;

use crate::currency_registrar::CurrencyRegistrarId;

/// Describes an currency registrar join request submission.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CurrencyRegistrarJoinRequestSubmission {
    pub currency_registrar_id: CurrencyRegistrarId,
    pub requester_id: UserId,
}

impl CurrencyRegistrarJoinRequestSubmission {
    pub(super) fn into_parts(self) -> (CurrencyRegistrarId, UserId) {
        (self.currency_registrar_id, self.requester_id)
    }
}
