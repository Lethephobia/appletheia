-- Currency and token-binding fragments
CREATE TABLE IF NOT EXISTS currency_fragments (
    id uuid PRIMARY KEY,
    currency_registrar_id uuid NOT NULL,
    code text NOT NULL UNIQUE,
    decimals smallint NOT NULL,
    description text,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT currency_fragments_code_check CHECK (code ~ '^[A-Z]+$'),
    CONSTRAINT currency_fragments_decimals_check CHECK (decimals BETWEEN 0 AND 255),
    CONSTRAINT currency_fragments_status_check CHECK (status IN ('defined', 'active', 'inactive'))
);

CREATE TABLE IF NOT EXISTS currency_token_binding_fragments (
    id uuid PRIMARY KEY,
    currency_id uuid NOT NULL REFERENCES currency_fragments(id) ON DELETE CASCADE,
    chain_network text NOT NULL,
    token_address text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL,
    CONSTRAINT currency_token_binding_fragments_token_unique UNIQUE (chain_network, token_address)
);

CREATE INDEX IF NOT EXISTS currency_token_binding_fragments_currency_idx
    ON currency_token_binding_fragments (currency_id);

CREATE INDEX IF NOT EXISTS currency_fragments_currency_registrar_id_idx
    ON currency_fragments (currency_registrar_id);

-- Shared account fragments
CREATE TABLE IF NOT EXISTS account_fragments (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    name text NOT NULL,
    description text,
    currency_id uuid NOT NULL REFERENCES currency_fragments(id),
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

CREATE INDEX IF NOT EXISTS account_fragments_currency_id_idx
    ON account_fragments (currency_id);

CREATE INDEX IF NOT EXISTS account_fragments_status_idx
    ON account_fragments (status);


CREATE INDEX IF NOT EXISTS account_fragments_public_status_created_at_idx
    ON account_fragments (status, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS account_fragments_public_status_currency_id_idx
    ON account_fragments (status, currency_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS account_fragments_public_status_id_idx
    ON account_fragments (status, id);

-- Wallet bookmark fragments
CREATE TABLE IF NOT EXISTS wallet_bookmark_fragments (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    display_name text,
    description text,
    token_owner_address text NOT NULL,
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
    amount numeric(39, 0) NOT NULL,
    note text,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    source_event_sequence bigint NOT NULL,
    updated_event_sequence bigint NOT NULL,
    source_event_id uuid NOT NULL,
    updated_event_id uuid NOT NULL
);

CREATE INDEX IF NOT EXISTS account_transaction_transfer_fragments_correlation_idx
    ON account_transaction_transfer_fragments (correlation_id);

CREATE TABLE IF NOT EXISTS account_transaction_fragments (
    id uuid PRIMARY KEY,
    transfer_id uuid,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    account_id uuid NOT NULL,
    counterparty_account_id uuid,
    token_binding_id uuid,
    chain_network text,
    token_address text,
    onchain_transaction_id text,
    amount numeric(39, 0) NOT NULL,
    note text,
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
        kind IN ('deposit', 'withdrawal', 'transfer')
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

CREATE INDEX IF NOT EXISTS account_transaction_fragments_status_idx
    ON account_transaction_fragments (status);
