-- Add up migration script here
CREATE TABLE IF NOT EXISTS accounts (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    name text NOT NULL,
    currency_id uuid NOT NULL,
    balance numeric(39, 0) NOT NULL,
    reserved_balance numeric(39, 0) NOT NULL,
    status text NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT accounts_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT accounts_status_check CHECK (status IN ('active', 'frozen', 'closed'))
);

CREATE INDEX IF NOT EXISTS accounts_owner_idx
    ON accounts (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS accounts_owner_created_at_idx
    ON accounts (owner_type, owner_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS accounts_currency_idx
    ON accounts (currency_id);

CREATE INDEX IF NOT EXISTS accounts_status_idx
    ON accounts (status);

CREATE TABLE IF NOT EXISTS currencies (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    symbol text NOT NULL,
    name text NOT NULL,
    decimals smallint NOT NULL,
    supply numeric(39, 0) NOT NULL,
    status text NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT currencies_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT currencies_status_check CHECK (status IN ('active', 'inactive'))
);

CREATE UNIQUE INDEX IF NOT EXISTS currencies_symbol_idx
    ON currencies (symbol);

CREATE INDEX IF NOT EXISTS currencies_owner_idx
    ON currencies (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS currencies_status_idx
    ON currencies (status);

CREATE TABLE IF NOT EXISTS currency_issuances (
    id uuid PRIMARY KEY,
    currency_id uuid NOT NULL,
    destination_account_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    status text NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT currency_issuances_status_check CHECK (
        status IN ('pending', 'completed', 'failed')
    )
);

CREATE INDEX IF NOT EXISTS currency_issuances_currency_idx
    ON currency_issuances (currency_id);

CREATE INDEX IF NOT EXISTS currency_issuances_destination_account_idx
    ON currency_issuances (destination_account_id);

CREATE INDEX IF NOT EXISTS currency_issuances_status_idx
    ON currency_issuances (status);

CREATE TABLE IF NOT EXISTS transfers (
    id uuid PRIMARY KEY,
    from_account_id uuid NOT NULL,
    to_account_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    status text NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT transfers_status_check CHECK (
        status IN ('pending', 'completed', 'failed', 'cancelled')
    )
);

CREATE INDEX IF NOT EXISTS transfers_from_account_idx
    ON transfers (from_account_id);

CREATE INDEX IF NOT EXISTS transfers_to_account_idx
    ON transfers (to_account_id);

CREATE INDEX IF NOT EXISTS transfers_status_idx
    ON transfers (status);
