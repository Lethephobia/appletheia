use appletheia::application::object_storage::{
    ObjectChecksum, ObjectContentLength, ObjectContentType,
};
use appletheia::command;
use banking_ledger_domain::currency::CurrencyId;
use serde::{Deserialize, Serialize};

/// Prepares a signed upload request for a currency image.
#[command(name = "currency_image_upload_prepare")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyImageUploadPrepareCommand {
    pub currency_id: CurrencyId,
    pub content_type: ObjectContentType,
    pub content_length: ObjectContentLength,
    pub checksum: ObjectChecksum,
}
