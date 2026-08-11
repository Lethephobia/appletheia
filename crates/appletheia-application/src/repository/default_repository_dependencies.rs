/// Collects infrastructure dependencies used by the default repository implementation.
pub struct DefaultRepositoryDependencies<ER, EW, EOE, SR, SW, UVOL, UKS, RIS, ESH> {
    pub event_reader: ER,
    pub event_writer: EW,
    pub event_outbox_enqueuer: EOE,
    pub snapshot_reader: SR,
    pub snapshot_writer: SW,
    pub unique_value_owner_lookup: UVOL,
    pub unique_key_reservation_store: UKS,
    pub reference_index_store: RIS,
    pub event_save_hook: ESH,
}
