/// Collects infrastructure dependencies used by the default repository implementation.
pub struct DefaultRepositoryDependencies<ER, EW, SR, SW, UVOL, UKS, ESH> {
    pub event_reader: ER,
    pub event_writer: EW,
    pub snapshot_reader: SR,
    pub snapshot_writer: SW,
    pub unique_value_owner_lookup: UVOL,
    pub unique_key_reservation_store: UKS,
    pub event_save_hook: ESH,
}
