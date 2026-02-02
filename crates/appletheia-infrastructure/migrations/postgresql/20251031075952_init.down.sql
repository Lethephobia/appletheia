-- projector processed events
DROP TABLE IF EXISTS projector_processed_events;

-- projection checkpoints
DROP TABLE IF EXISTS projection_checkpoints;

-- saga processed events
DROP TABLE IF EXISTS saga_processed_events;

-- saga instances
DROP TABLE IF EXISTS saga_instances;

-- idempotency
DROP TABLE IF EXISTS idempotency;

-- command dead letters
DROP TABLE IF EXISTS command_dead_letters;

-- command_outbox
DROP TABLE IF EXISTS command_outbox;

-- event dead letters
DROP TABLE IF EXISTS event_dead_letters;

-- event_outbox
DROP TABLE IF EXISTS event_outbox;

-- snapshots
DROP TABLE IF EXISTS snapshots;

-- events
DROP TABLE IF EXISTS events;
