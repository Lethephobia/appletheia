CREATE TABLE IF NOT EXISTS organizations (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    handle text NOT NULL,
    display_name text NOT NULL,
    description text,
    website_url text,
    picture jsonb,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT organizations_owner_type_check CHECK (owner_type IN ('user'))
);

CREATE UNIQUE INDEX IF NOT EXISTS organizations_handle_idx
    ON organizations (handle);

CREATE INDEX IF NOT EXISTS organizations_owner_idx
    ON organizations (owner_type, owner_id);
