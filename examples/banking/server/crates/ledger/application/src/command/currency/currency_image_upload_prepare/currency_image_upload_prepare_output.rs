use appletheia::application::object_storage::SignedObjectUpload;
use banking_ledger_domain::currency::CurrencyImageRef;
use serde::{Deserialize, Serialize};

use super::CurrencyImageUploadPrepareRejectionReason;

/// The output returned after preparing a currency-image upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum CurrencyImageUploadPrepareOutput {
    Prepared {
        image: CurrencyImageRef,
        signed_upload: Box<SignedObjectUpload>,
    },
    Rejected {
        reason: CurrencyImageUploadPrepareRejectionReason,
    },
}
