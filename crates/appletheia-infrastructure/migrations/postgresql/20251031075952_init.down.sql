-- auth token revocation cutoffs
DROP TABLE IF EXISTS auth_token_revocation_cutoffs;

-- auth token revocations
DROP TABLE IF EXISTS auth_token_revocations;

-- relationships (Aggregate × ReBAC)
DROP TABLE IF EXISTS relationships;

-- oidc login attempts
DROP TABLE IF EXISTS oidc_login_attempts;

-- resource response cache
DROP TABLE IF EXISTS resource_response_cache;

-- idempotency
DROP TABLE IF EXISTS idempotency;

-- projector processed events
DROP TABLE IF EXISTS projector_processed_events;

-- projection checkpoints
DROP TABLE IF EXISTS projection_checkpoints;

-- saga processed events
DROP TABLE IF EXISTS saga_processed_events;

-- saga instances
DROP TABLE IF EXISTS saga_instances;

-- command dead letters
DROP TABLE IF EXISTS command_dead_letters;

-- command_outbox
DROP TABLE IF EXISTS command_outbox;

-- event dead letters
DROP TABLE IF EXISTS event_dead_letters;

-- event_outbox
DROP TABLE IF EXISTS event_outbox;

-- unique key reservations
DROP TABLE IF EXISTS unique_key_reservations;

-- snapshots
DROP TABLE IF EXISTS snapshots;

-- events
DROP TABLE IF EXISTS events;
