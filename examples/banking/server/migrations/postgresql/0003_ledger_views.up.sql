-- Shared account fragments
CREATE TABLE IF NOT EXISTS account_fragments (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    name text NOT NULL,
    currency_id uuid NOT NULL,
    balance numeric(39, 0) NOT NULL,
    reserved_balance numeric(39, 0) NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT account_fragments_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT account_fragments_status_check CHECK (status IN ('active', 'frozen'))
);

CREATE INDEX IF NOT EXISTS account_fragments_owner_idx
    ON account_fragments (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS account_fragments_owner_created_at_idx
    ON account_fragments (owner_type, owner_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS account_fragments_currency_idx
    ON account_fragments (currency_id);

CREATE INDEX IF NOT EXISTS account_fragments_status_idx
    ON account_fragments (status);


CREATE INDEX IF NOT EXISTS account_fragments_public_status_created_at_idx
    ON account_fragments (status, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS account_fragments_public_status_currency_idx
    ON account_fragments (status, currency_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS account_fragments_public_status_id_idx
    ON account_fragments (status, id);

-- Shared currency fragments
CREATE TABLE IF NOT EXISTS currency_fragments (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    symbol text NOT NULL,
    name text NOT NULL,
    decimals smallint NOT NULL,
    description text,
    image_type text,
    image_object_name text,
    image_external_url text,
    mint_account_address text,
    supply numeric(39, 0) NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT currency_fragments_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT currency_fragments_status_check CHECK (status IN ('provisioning', 'active', 'inactive', 'provisioning_failed')),
    CONSTRAINT currency_fragments_image_check CHECK (
        (image_type IS NULL AND image_object_name IS NULL AND image_external_url IS NULL)
        OR (
            image_type IS NOT NULL
            AND image_type = 'object_name'
            AND image_object_name IS NOT NULL
            AND image_external_url IS NULL
        )
        OR (
            image_type IS NOT NULL
            AND image_type = 'external_url'
            AND image_object_name IS NULL
            AND image_external_url IS NOT NULL
        )
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS currency_fragments_symbol_idx
    ON currency_fragments (symbol);

CREATE INDEX IF NOT EXISTS currency_fragments_owner_idx
    ON currency_fragments (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS currency_fragments_status_idx
    ON currency_fragments (status);

CREATE INDEX IF NOT EXISTS currency_fragments_created_at_idx
    ON currency_fragments (created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS currency_fragments_status_created_at_idx
    ON currency_fragments (status, created_at DESC, id DESC);

-- Wallet bookmark fragments
CREATE TABLE IF NOT EXISTS wallet_bookmark_fragments (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    display_name text,
    description text,
    token_account_owner_address text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT wallet_bookmark_fragments_owner_type_check CHECK (owner_type IN ('user', 'organization'))
);

CREATE INDEX IF NOT EXISTS wallet_bookmark_fragments_owner_idx
    ON wallet_bookmark_fragments (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS wallet_bookmark_fragments_owner_created_at_idx
    ON wallet_bookmark_fragments (owner_type, owner_id, created_at DESC, id DESC);

-- Account transaction fragments
CREATE TABLE IF NOT EXISTS account_transaction_transfer_fragments (
    id uuid PRIMARY KEY,
    correlation_id uuid NOT NULL,
    from_account_id uuid NOT NULL,
    to_account_id uuid NOT NULL,
    currency_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL
);

CREATE INDEX IF NOT EXISTS account_transaction_transfer_fragments_correlation_idx
    ON account_transaction_transfer_fragments (correlation_id);

CREATE TABLE IF NOT EXISTS account_transaction_currency_issuance_fragments (
    id uuid PRIMARY KEY,
    destination_account_id uuid NOT NULL,
    currency_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL
);

CREATE TABLE IF NOT EXISTS account_transaction_fragments (
    id uuid PRIMARY KEY,
    transfer_id uuid,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    account_id uuid NOT NULL,
    counterparty_account_id uuid,
    currency_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    direction text NOT NULL,
    kind text NOT NULL,
    status text NOT NULL,
    occurred_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT account_transaction_fragments_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT account_transaction_fragments_direction_check CHECK (direction IN ('incoming', 'outgoing')),
    CONSTRAINT account_transaction_fragments_kind_check CHECK (
        kind IN ('deposit', 'withdrawal', 'transfer', 'currency_issuance')
    ),
    CONSTRAINT account_transaction_fragments_status_check CHECK (
        status IN ('pending', 'completed', 'failed', 'requires_review')
    )
);

CREATE INDEX IF NOT EXISTS account_transaction_fragments_owner_idx
    ON account_transaction_fragments (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS account_transaction_fragments_owner_occurred_at_idx
    ON account_transaction_fragments (owner_type, owner_id, occurred_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS account_transaction_fragments_account_idx
    ON account_transaction_fragments (account_id);

CREATE INDEX IF NOT EXISTS account_transaction_fragments_currency_idx
    ON account_transaction_fragments (currency_id);

CREATE INDEX IF NOT EXISTS account_transaction_fragments_status_idx
    ON account_transaction_fragments (status);
