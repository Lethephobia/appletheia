#[derive(Debug, thiserror::Error)]
pub enum PgUniqueKeyReservationStoreError {
    #[error("unique key conflict could not be identified")]
    UnidentifiedConflict,
}
