-- events
CREATE TABLE IF NOT EXISTS events (
  id                  UUID PRIMARY KEY,
  aggregate_type      TEXT        NOT NULL,
  aggregate_id        UUID        NOT NULL,
  aggregate_version   INT         NOT NULL CHECK (aggregate_version > 0),
  payload             JSONB       NOT NULL,
  created_at          TIMESTAMPTZ NOT NULL,
  CONSTRAINT events_uniq_aggregate_version
    UNIQUE (aggregate_type, aggregate_id, aggregate_version)
);

CREATE INDEX IF NOT EXISTS idx_events_created_at
  ON events (created_at);

-- snapshots
CREATE TABLE IF NOT EXISTS snapshots (
  id                  UUID PRIMARY KEY,
  aggregate_type      TEXT        NOT NULL,
  aggregate_id        UUID        NOT NULL,
  aggregate_version   INT         NOT NULL CHECK (aggregate_version > 0),
  state               JSONB       NOT NULL,
  created_at          TIMESTAMPTZ NOT NULL,
  CONSTRAINT snapshots_uniq_aggregate_version
    UNIQUE (aggregate_type, aggregate_id, aggregate_version)
);

CREATE INDEX IF NOT EXISTS idx_snapshots_created_at
  ON snapshots (created_at);

-- comments
COMMENT ON TABLE events    IS 'Event store: append-only; one (aggregate_type, aggregate_id, version) is unique.';
COMMENT ON TABLE snapshots IS 'Materialized snapshots per aggregate version; latest is fetched via DESC index.';
