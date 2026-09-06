-- auth token exchange codes
DROP TABLE IF EXISTS auth_token_exchange_codes;

-- auth token revocation cutoffs
DROP TABLE IF EXISTS auth_token_revocation_cutoffs;

-- auth token revocations
DROP TABLE IF EXISTS auth_token_revocations;

-- relationships (Aggregate × ReBAC)
DROP TABLE IF EXISTS relationships;

-- oidc continuations
DROP TABLE IF EXISTS oidc_continuations;

-- oidc login attempts
DROP TABLE IF EXISTS oidc_login_attempts;

-- resource response cache
DROP TABLE IF EXISTS resource_response_cache;

-- read model invalidation outbox
DROP TABLE IF EXISTS read_model_invalidation_outbox;

-- projector processed events
DROP TABLE IF EXISTS projector_processed_events;

-- projection checkpoints
DROP TABLE IF EXISTS projection_checkpoints;

-- saga processed events
DROP TABLE IF EXISTS saga_processed_events;

-- saga processed command failures
DROP TABLE IF EXISTS saga_processed_command_failures;

-- saga instances
DROP TABLE IF EXISTS saga_dispatched_commands;
DROP TABLE IF EXISTS saga_instances;

-- idempotency
DROP TABLE IF EXISTS idempotency;

-- command failure dead letters
DROP TABLE IF EXISTS command_failure_dead_letters;

-- command failure outbox and executions
DROP TABLE IF EXISTS command_failure_outbox;
DROP TABLE IF EXISTS command_executions;

-- command dead letters
DROP TABLE IF EXISTS command_dead_letters;

-- command outbox
DROP TABLE IF EXISTS command_outbox;

-- event dead letters
DROP TABLE IF EXISTS event_dead_letters;

-- event outbox
DROP TABLE IF EXISTS event_outbox;

-- aggregate reference indexes
DROP INDEX IF EXISTS idx_aggregate_reference_indexes_lookup_page;
DROP TABLE IF EXISTS aggregate_reference_indexes;

-- unique key reservations
DROP TABLE IF EXISTS unique_key_reservations;

-- snapshots
DROP TABLE IF EXISTS snapshots;

-- events
DROP TABLE IF EXISTS events;
