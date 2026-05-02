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

CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY,
    username text,
    display_name text,
    bio text,
    picture jsonb,
    status text NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT users_status_check CHECK (status IN ('active', 'inactive'))
);

CREATE UNIQUE INDEX IF NOT EXISTS users_username_idx
    ON users (username)
    WHERE username IS NOT NULL;

CREATE INDEX IF NOT EXISTS users_status_idx
    ON users (status);

CREATE TABLE IF NOT EXISTS user_identities (
    provider text NOT NULL,
    subject text NOT NULL,
    user_id uuid NOT NULL,
    email text,
    updated_event_sequence bigint NOT NULL,
    PRIMARY KEY (provider, subject)
);

CREATE INDEX IF NOT EXISTS user_identities_user_idx
    ON user_identities (user_id);

CREATE INDEX IF NOT EXISTS user_identities_email_idx
    ON user_identities (email)
    WHERE email IS NOT NULL;

CREATE TABLE IF NOT EXISTS organization_join_requests (
    id uuid PRIMARY KEY,
    organization_id uuid NOT NULL,
    requester_id uuid NOT NULL,
    status text NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT organization_join_requests_status_check CHECK (
        status IN ('pending', 'approved', 'rejected', 'canceled')
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS organization_join_requests_pending_organization_requester_idx
    ON organization_join_requests (organization_id, requester_id)
    WHERE status = 'pending';

CREATE INDEX IF NOT EXISTS organization_join_requests_requester_idx
    ON organization_join_requests (requester_id);

CREATE INDEX IF NOT EXISTS organization_join_requests_status_idx
    ON organization_join_requests (status);

CREATE TABLE IF NOT EXISTS organization_invitations (
    id uuid PRIMARY KEY,
    organization_id uuid NOT NULL,
    invitee_id uuid NOT NULL,
    issuer_type text NOT NULL,
    issuer_id uuid,
    expires_at timestamptz NOT NULL,
    status text NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT organization_invitations_issuer_type_check CHECK (
        issuer_type IN ('user', 'system')
    ),
    CONSTRAINT organization_invitations_issuer_check CHECK (
        (issuer_type = 'user' AND issuer_id IS NOT NULL)
        OR (issuer_type = 'system' AND issuer_id IS NULL)
    ),
    CONSTRAINT organization_invitations_status_check CHECK (
        status IN ('pending', 'accepted', 'declined', 'canceled', 'rejected')
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS organization_invitations_pending_organization_invitee_idx
    ON organization_invitations (organization_id, invitee_id)
    WHERE status = 'pending';

CREATE INDEX IF NOT EXISTS organization_invitations_invitee_idx
    ON organization_invitations (invitee_id);

CREATE INDEX IF NOT EXISTS organization_invitations_issuer_idx
    ON organization_invitations (issuer_type, issuer_id);

CREATE INDEX IF NOT EXISTS organization_invitations_status_idx
    ON organization_invitations (status);

CREATE TABLE IF NOT EXISTS organization_memberships (
    id uuid PRIMARY KEY,
    organization_id uuid NOT NULL,
    user_id uuid NOT NULL,
    status text NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT organization_memberships_status_check CHECK (status IN ('active', 'inactive'))
);

CREATE UNIQUE INDEX IF NOT EXISTS organization_memberships_organization_user_idx
    ON organization_memberships (organization_id, user_id);

CREATE INDEX IF NOT EXISTS organization_memberships_user_idx
    ON organization_memberships (user_id);

CREATE INDEX IF NOT EXISTS organization_memberships_status_idx
    ON organization_memberships (status);

CREATE TABLE IF NOT EXISTS organization_membership_roles (
    organization_membership_id uuid NOT NULL,
    role text NOT NULL,
    updated_event_sequence bigint NOT NULL,
    PRIMARY KEY (organization_membership_id, role),
    CONSTRAINT organization_membership_roles_role_check CHECK (
        role IN ('admin', 'finance_manager', 'treasurer')
    )
);

CREATE INDEX IF NOT EXISTS organization_membership_roles_role_idx
    ON organization_membership_roles (role);
