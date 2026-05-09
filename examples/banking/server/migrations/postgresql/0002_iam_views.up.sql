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
    updated_event_sequence bigint NOT NULL,
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
    updated_event_sequence bigint NOT NULL,
    PRIMARY KEY (user_id, provider, subject)
);

CREATE INDEX IF NOT EXISTS user_private_info_identities_user_idx
    ON user_private_info_identities (user_id);

CREATE INDEX IF NOT EXISTS user_private_info_identities_email_idx
    ON user_private_info_identities (email)
    WHERE email IS NOT NULL;

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
    updated_event_sequence bigint NOT NULL,
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
