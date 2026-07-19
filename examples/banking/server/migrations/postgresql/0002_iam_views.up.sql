-- user_private_info read model
CREATE TABLE IF NOT EXISTS user_private_infos (
    id uuid PRIMARY KEY,
    username text,
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
    CONSTRAINT user_private_infos_status_check CHECK (status IN ('active', 'inactive')),
    CONSTRAINT user_private_infos_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS user_private_infos_username_idx
    ON user_private_infos (username)
    WHERE username IS NOT NULL;

CREATE INDEX IF NOT EXISTS user_private_infos_status_idx
    ON user_private_infos (status);

CREATE TABLE IF NOT EXISTS user_private_info_identities (
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

CREATE INDEX IF NOT EXISTS user_private_info_identities_user_idx
    ON user_private_info_identities (user_id);

CREATE INDEX IF NOT EXISTS user_private_info_identities_email_idx
    ON user_private_info_identities (email)
    WHERE email IS NOT NULL;

CREATE TABLE IF NOT EXISTS user_private_info_organizations (
    id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_private_info_organizations_picture_check CHECK (
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

CREATE INDEX IF NOT EXISTS user_private_info_organizations_handle_idx
    ON user_private_info_organizations (handle);

CREATE TABLE IF NOT EXISTS user_private_info_organization_memberships (
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

CREATE INDEX IF NOT EXISTS user_private_info_organization_memberships_user_idx
    ON user_private_info_organization_memberships (user_id);

CREATE INDEX IF NOT EXISTS user_private_info_organization_memberships_organization_idx
    ON user_private_info_organization_memberships (organization_id);

-- public_user_list read model
CREATE TABLE IF NOT EXISTS public_user_list_items (
    id uuid PRIMARY KEY,
    username text,
    username_search_text text GENERATED ALWAYS AS (
        regexp_replace(lower(username), '[[:space:]]+', '', 'g')
    ) STORED,
    display_name text,
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
    CONSTRAINT public_user_list_items_status_check CHECK (status IN ('active', 'inactive')),
    CONSTRAINT public_user_list_items_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS public_user_list_items_username_idx
    ON public_user_list_items (username)
    WHERE username IS NOT NULL;

CREATE INDEX IF NOT EXISTS public_user_list_items_username_contains_idx
    ON public_user_list_items USING gin (username_search_text gin_bigm_ops)
    WHERE username_search_text IS NOT NULL;

CREATE INDEX IF NOT EXISTS public_user_list_items_status_created_at_idx
    ON public_user_list_items (status, created_at DESC, id DESC);

-- public_organization_list read model
CREATE TABLE IF NOT EXISTS public_organization_list_items (
    id uuid PRIMARY KEY,
    handle text NOT NULL,
    handle_search_text text GENERATED ALWAYS AS (
        regexp_replace(lower(handle), '[[:space:]]+', '', 'g')
    ) STORED,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT public_organization_list_items_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS public_organization_list_items_handle_idx
    ON public_organization_list_items (handle);

CREATE INDEX IF NOT EXISTS public_organization_list_items_handle_contains_idx
    ON public_organization_list_items USING gin (handle_search_text gin_bigm_ops)
    WHERE handle_search_text IS NOT NULL;

CREATE INDEX IF NOT EXISTS public_organization_list_items_created_at_idx
    ON public_organization_list_items (created_at DESC, id DESC);

-- organization_internal_info read model
CREATE TABLE IF NOT EXISTS organization_internal_infos (
    id uuid PRIMARY KEY,
    handle text NOT NULL,
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
    CONSTRAINT organization_internal_infos_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS organization_internal_infos_handle_idx
    ON organization_internal_infos (handle);

-- organization_management_info read model
CREATE TABLE IF NOT EXISTS organization_management_info_owner_users (
    user_id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_management_info_owner_users_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS organization_management_info_owner_users_username_idx
    ON organization_management_info_owner_users (username)
    WHERE username IS NOT NULL;

CREATE TABLE IF NOT EXISTS organization_management_infos (
    id uuid PRIMARY KEY,
    owner_user_id uuid NOT NULL,
    handle text NOT NULL,
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
    CONSTRAINT organization_management_infos_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS organization_management_infos_handle_idx
    ON organization_management_infos (handle);

CREATE INDEX IF NOT EXISTS organization_management_infos_owner_user_idx
    ON organization_management_infos (owner_user_id);

-- organization_member_list read model
CREATE TABLE IF NOT EXISTS organization_member_list_organizations (
    organization_id uuid PRIMARY KEY,
    owner_user_id uuid NOT NULL,
    owner_since timestamptz NOT NULL,
    owner_source_event_id uuid NOT NULL,
    owner_updated_event_id uuid NOT NULL,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_member_list_orgs_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS organization_member_list_users (
    user_id uuid PRIMARY KEY,
    username text,
    username_search_text text GENERATED ALWAYS AS (
        regexp_replace(lower(username), '[[:space:]]+', '', 'g')
    ) STORED,
    display_name text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_member_list_users_picture_check CHECK (
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

CREATE INDEX IF NOT EXISTS organization_member_list_users_username_contains_idx
    ON organization_member_list_users USING gin (username_search_text gin_bigm_ops)
    WHERE username_search_text IS NOT NULL;

CREATE TABLE IF NOT EXISTS organization_member_list_memberships (
    organization_id uuid NOT NULL,
    user_id uuid NOT NULL,
    roles jsonb NOT NULL,
    joined_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    PRIMARY KEY (organization_id, user_id)
);

CREATE INDEX IF NOT EXISTS organization_member_list_memberships_scope_joined_at_idx
    ON organization_member_list_memberships (organization_id, joined_at DESC, user_id DESC);

CREATE INDEX IF NOT EXISTS organization_member_list_memberships_user_idx
    ON organization_member_list_memberships (user_id);

-- organization_invitation_list read model
CREATE TABLE IF NOT EXISTS organization_invitation_list_organizations (
    organization_id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_invitation_list_orgs_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS organization_invitation_list_users (
    user_id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_invitation_list_users_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS organization_invitation_list_items (
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
    CONSTRAINT organization_invitation_list_items_status_check CHECK (
        status IN ('pending', 'accepted', 'declined', 'canceled', 'rejected')
    ),
    CONSTRAINT organization_invitation_list_items_issuer_check CHECK (
        (issuer_type = 'user' AND issuer_user_id IS NOT NULL)
        OR (issuer_type = 'system' AND issuer_user_id IS NULL)
    )
);

CREATE INDEX IF NOT EXISTS organization_invitation_list_scope_created_at_idx
    ON organization_invitation_list_items (organization_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_invitation_list_scope_status_idx
    ON organization_invitation_list_items (organization_id, status, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_invitation_list_invitee_idx
    ON organization_invitation_list_items (invitee_user_id);

-- user_organization_invitation_list read model
CREATE TABLE IF NOT EXISTS user_organization_invitation_list_users (
    user_id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_organization_invitation_list_users_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS user_organization_invitation_list_organizations (
    organization_id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_org_invitation_list_orgs_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS user_organization_invitation_list_items (
    id uuid PRIMARY KEY,
    invitee_user_id uuid NOT NULL,
    organization_id uuid NOT NULL,
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
    CONSTRAINT user_org_invitation_list_items_status_check CHECK (
        status IN ('pending', 'accepted', 'declined', 'canceled', 'rejected')
    ),
    CONSTRAINT user_org_invitation_list_items_issuer_check CHECK (
        (issuer_type = 'user' AND issuer_user_id IS NOT NULL)
        OR (issuer_type = 'system' AND issuer_user_id IS NULL)
    )
);

CREATE INDEX IF NOT EXISTS user_org_invitation_list_scope_created_at_idx
    ON user_organization_invitation_list_items (invitee_user_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS user_org_invitation_list_scope_status_idx
    ON user_organization_invitation_list_items (
        invitee_user_id, status, created_at DESC, id DESC
    );

CREATE INDEX IF NOT EXISTS user_organization_invitation_list_org_idx
    ON user_organization_invitation_list_items (organization_id);

-- organization_join_request_list read model
CREATE TABLE IF NOT EXISTS organization_join_request_list_organizations (
    organization_id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_join_request_list_orgs_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS organization_join_request_list_users (
    user_id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT organization_join_request_list_users_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS organization_join_request_list_items (
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
    CONSTRAINT organization_join_request_list_items_status_check CHECK (
        status IN ('pending', 'approved', 'rejected', 'canceled')
    )
);

CREATE INDEX IF NOT EXISTS organization_join_request_list_scope_created_at_idx
    ON organization_join_request_list_items (organization_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS organization_join_request_list_scope_status_idx
    ON organization_join_request_list_items (
        organization_id, status, created_at DESC, id DESC
    );

CREATE INDEX IF NOT EXISTS organization_join_request_list_requester_idx
    ON organization_join_request_list_items (requester_user_id);

-- user_organization_join_request_list read model
CREATE TABLE IF NOT EXISTS user_organization_join_request_list_users (
    user_id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_organization_join_request_list_users_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS user_organization_join_request_list_organizations (
    organization_id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture_type text,
    picture_object_name text,
    picture_external_url text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_org_join_request_list_orgs_picture_check CHECK (
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

CREATE TABLE IF NOT EXISTS user_organization_join_request_list_items (
    id uuid PRIMARY KEY,
    requester_user_id uuid NOT NULL,
    organization_id uuid NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT user_org_join_request_list_items_status_check CHECK (
        status IN ('pending', 'approved', 'rejected', 'canceled')
    )
);

CREATE INDEX IF NOT EXISTS user_org_join_request_list_scope_created_at_idx
    ON user_organization_join_request_list_items (requester_user_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS user_org_join_request_list_scope_status_idx
    ON user_organization_join_request_list_items (
        requester_user_id, status, created_at DESC, id DESC
    );

CREATE INDEX IF NOT EXISTS user_organization_join_request_list_org_idx
    ON user_organization_join_request_list_items (organization_id);

-- user_public_profile read model
CREATE TABLE IF NOT EXISTS user_public_profiles (
    id uuid PRIMARY KEY,
    username text,
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
    CONSTRAINT user_public_profiles_status_check CHECK (status IN ('active', 'inactive')),
    CONSTRAINT user_public_profiles_picture_check CHECK (
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

CREATE UNIQUE INDEX IF NOT EXISTS user_public_profiles_username_idx
    ON user_public_profiles (username)
    WHERE username IS NOT NULL;

CREATE INDEX IF NOT EXISTS user_public_profiles_status_idx
    ON user_public_profiles (status);
