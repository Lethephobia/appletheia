-- Shared user identity fragments
CREATE TABLE IF NOT EXISTS user_identity_fragments (
    user_id uuid NOT NULL,
    provider text NOT NULL,
    subject text NOT NULL,
    email text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    PRIMARY KEY (user_id, provider, subject)
);

CREATE INDEX IF NOT EXISTS user_identity_fragments_user_idx
    ON user_identity_fragments (user_id);

CREATE INDEX IF NOT EXISTS user_identity_fragments_email_idx
    ON user_identity_fragments (email)
    WHERE email IS NOT NULL;


-- Shared organization membership fragments
CREATE TABLE IF NOT EXISTS organization_membership_fragments (
    organization_membership_id uuid NOT NULL,
    user_id uuid NOT NULL,
    organization_id uuid NOT NULL,
    roles jsonb NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    PRIMARY KEY (user_id, organization_id)
);

CREATE INDEX IF NOT EXISTS organization_membership_fragments_user_idx
    ON organization_membership_fragments (user_id);

CREATE INDEX IF NOT EXISTS organization_membership_fragments_organization_idx
    ON organization_membership_fragments (organization_id);

-- shared public user fragment
CREATE TABLE IF NOT EXISTS user_fragments (
    id uuid PRIMARY KEY,
    username text,
    username_search_text text GENERATED ALWAYS AS (
        regexp_replace(lower(username), '[[:space:]]+', '', 'g')
    ) STORED,
    display_name text,
    bio text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_fragments_status_check CHECK (status IN ('active', 'inactive')),
    CONSTRAINT user_fragments_picture_check CHECK (
        (picture_type IS NULL AND picture_object_name IS NULL AND picture_external_url IS NULL)
        OR (
            picture_type IS NOT NULL
            AND picture_type = 'object_name'
            AND picture_object_name IS NOT NULL
            AND picture_external_url IS NULL
        )
        OR (
            picture_type IS NOT NULL
            AND picture_type = 'external_url'
            AND picture_object_name IS NULL
            AND picture_external_url IS NOT NULL
        )
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS user_fragments_username_idx
    ON user_fragments (username)
    WHERE username IS NOT NULL;

CREATE INDEX IF NOT EXISTS user_fragments_username_contains_idx
    ON user_fragments USING gin (username_search_text gin_bigm_ops)
    WHERE username_search_text IS NOT NULL;

CREATE INDEX IF NOT EXISTS user_fragments_status_created_at_idx
    ON user_fragments (status, created_at DESC, id DESC);

CREATE TABLE IF NOT EXISTS user_fragment_tombstones (
    user_id uuid PRIMARY KEY,
    event_sequence bigint NOT NULL,
    event_id uuid NOT NULL,
    occurred_at timestamptz NOT NULL
);

-- Shared organization fragments
CREATE TABLE IF NOT EXISTS organization_fragments (
    id uuid PRIMARY KEY,
    owner_user_id uuid NOT NULL,
    owner_since timestamptz NOT NULL,
    owner_source_event_id uuid NOT NULL,
    owner_updated_event_id uuid NOT NULL,
    handle text NOT NULL,
    handle_search_text text GENERATED ALWAYS AS (
        regexp_replace(lower(handle), '[[:space:]]+', '', 'g')
    ) STORED,
    display_name text NOT NULL,
    description text,
    website_url text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_fragments_picture_check CHECK (
        (picture_type IS NULL AND picture_object_name IS NULL AND picture_external_url IS NULL)
        OR (
            picture_type IS NOT NULL
            AND picture_type = 'object_name'
            AND picture_object_name IS NOT NULL
            AND picture_external_url IS NULL
        )
        OR (
            picture_type IS NOT NULL
            AND picture_type = 'external_url'
            AND picture_object_name IS NULL
            AND picture_external_url IS NOT NULL
        )
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS organization_fragments_handle_idx
    ON organization_fragments (handle);

CREATE INDEX IF NOT EXISTS organization_fragments_owner_user_idx
    ON organization_fragments (owner_user_id);


CREATE INDEX IF NOT EXISTS organization_fragments_handle_contains_idx
    ON organization_fragments USING gin (handle_search_text gin_bigm_ops)
    WHERE handle_search_text IS NOT NULL;

CREATE INDEX IF NOT EXISTS organization_fragments_created_at_idx
    ON organization_fragments (created_at DESC, id DESC);

-- Shared organization invitation fragments
CREATE TABLE IF NOT EXISTS organization_invitation_fragments (
    id uuid PRIMARY KEY,
    organization_id uuid NOT NULL,
    invitee_user_id uuid NOT NULL,
    roles jsonb NOT NULL,
    issuer_type text NOT NULL,
    issuer_user_id uuid,
    expires_at timestamptz NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_invitation_fragments_status_check CHECK (
        status IN ('pending', 'accepted', 'declined', 'canceled', 'rejected')
    ),
    CONSTRAINT organization_invitation_fragments_issuer_check CHECK (
        (issuer_type = 'user' AND issuer_user_id IS NOT NULL)
        OR (issuer_type = 'system' AND issuer_user_id IS NULL)
    )
);

CREATE INDEX IF NOT EXISTS organization_invitation_fragments_organization_created_at_idx
    ON organization_invitation_fragments (organization_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_invitation_fragments_organization_status_idx
    ON organization_invitation_fragments (
        organization_id, status, created_at DESC, id DESC
    );

CREATE INDEX IF NOT EXISTS organization_invitation_fragments_invitee_created_at_idx
    ON organization_invitation_fragments (invitee_user_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_invitation_fragments_invitee_status_idx
    ON organization_invitation_fragments (
        invitee_user_id, status, created_at DESC, id DESC
    );

-- Shared organization join request fragments
CREATE TABLE IF NOT EXISTS organization_join_request_fragments (
    id uuid PRIMARY KEY,
    organization_id uuid NOT NULL,
    requester_user_id uuid NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_join_request_fragments_status_check CHECK (
        status IN ('pending', 'approved', 'rejected', 'canceled')
    )
);

CREATE INDEX IF NOT EXISTS organization_join_request_fragments_organization_created_at_idx
    ON organization_join_request_fragments (organization_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_join_request_fragments_organization_status_idx
    ON organization_join_request_fragments (
        organization_id, status, created_at DESC, id DESC
    );

CREATE INDEX IF NOT EXISTS organization_join_request_fragments_requester_created_at_idx
    ON organization_join_request_fragments (requester_user_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_join_request_fragments_requester_status_idx
    ON organization_join_request_fragments (
        requester_user_id, status, created_at DESC, id DESC
    );
