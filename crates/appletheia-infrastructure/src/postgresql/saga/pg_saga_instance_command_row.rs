use sqlx::FromRow;
use uuid::Uuid;

use appletheia_application::request_context::MessageId;

#[derive(Debug, FromRow)]
pub struct PgSagaInstanceCommandRow {
    pub message_id: Uuid,
}

impl PgSagaInstanceCommandRow {
    pub fn into_message_id(self) -> MessageId {
        MessageId::from(self.message_id)
    }
}
