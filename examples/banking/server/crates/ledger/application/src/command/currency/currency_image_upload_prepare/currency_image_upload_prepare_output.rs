use appletheia::application::object_storage::SignedObjectUpload;
use banking_ledger_domain::currency::CurrencyImageRef;
use serde::{Deserialize, Serialize};

/// The output returned after preparing a currency-image upload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyImageUploadPrepareOutput {
    pub image: CurrencyImageRef,
    pub signed_upload: SignedObjectUpload,
}

impl CurrencyImageUploadPrepareOutput {
    /// Creates a new currency-image-upload-prepare output.
    pub fn new(image: CurrencyImageRef, signed_upload: SignedObjectUpload) -> Self {
        Self {
            image,
            signed_upload,
        }
    }
}
