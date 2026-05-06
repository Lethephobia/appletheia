-- owned_account_list_item read model
CREATE TABLE IF NOT EXISTS owned_account_list_item_currencies (
    id uuid PRIMARY KEY,
    symbol text NOT NULL,
    name text NOT NULL,
    decimals smallint NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS owned_account_list_item_currencies_symbol_idx
    ON owned_account_list_item_currencies (symbol);

CREATE TABLE IF NOT EXISTS owned_account_list_items (
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
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT owned_account_list_items_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT owned_account_list_items_status_check CHECK (status IN ('active', 'frozen'))
);

CREATE INDEX IF NOT EXISTS owned_account_list_items_owner_idx
    ON owned_account_list_items (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS owned_account_list_items_owner_created_at_idx
    ON owned_account_list_items (owner_type, owner_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS owned_account_list_items_currency_idx
    ON owned_account_list_items (currency_id);

CREATE INDEX IF NOT EXISTS owned_account_list_items_status_idx
    ON owned_account_list_items (status);

-- currency_list_item read model
CREATE TABLE IF NOT EXISTS currency_list_item_owner_users (
    id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture jsonb,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS currency_list_item_owner_organizations (
    id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture jsonb,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS currency_list_items (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    symbol text NOT NULL,
    name text NOT NULL,
    decimals smallint NOT NULL,
    supply numeric(39, 0) NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT currency_list_items_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT currency_list_items_status_check CHECK (status IN ('active', 'inactive'))
);

CREATE UNIQUE INDEX IF NOT EXISTS currency_list_items_symbol_idx
    ON currency_list_items (symbol);

CREATE INDEX IF NOT EXISTS currency_list_items_owner_idx
    ON currency_list_items (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS currency_list_items_status_idx
    ON currency_list_items (status);

CREATE INDEX IF NOT EXISTS currency_list_items_created_at_idx
    ON currency_list_items (created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS currency_list_items_status_created_at_idx
    ON currency_list_items (status, created_at DESC, id DESC);

-- owned_account_transaction_list_item read model
CREATE TABLE IF NOT EXISTS owned_account_transaction_list_item_currencies (
    id uuid PRIMARY KEY,
    symbol text NOT NULL,
    name text NOT NULL,
    decimals smallint NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS owned_account_transaction_list_item_currencies_symbol_idx
    ON owned_account_transaction_list_item_currencies (symbol);

CREATE TABLE IF NOT EXISTS owned_account_transaction_list_item_owner_users (
    id uuid PRIMARY KEY,
    username text,
    display_name text,
    picture jsonb,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS owned_account_transaction_list_item_owner_organizations (
    id uuid PRIMARY KEY,
    handle text NOT NULL,
    display_name text NOT NULL,
    picture jsonb,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS owned_account_transaction_list_transfers (
    id uuid PRIMARY KEY,
    correlation_id uuid NOT NULL,
    from_account_id uuid NOT NULL,
    to_account_id uuid NOT NULL,
    currency_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE INDEX IF NOT EXISTS owned_account_transaction_list_transfers_correlation_idx
    ON owned_account_transaction_list_transfers (correlation_id);

CREATE TABLE IF NOT EXISTS owned_account_transaction_list_currency_issuances (
    id uuid PRIMARY KEY,
    destination_account_id uuid NOT NULL,
    currency_id uuid NOT NULL,
    amount numeric(39, 0) NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS owned_account_transaction_list_items (
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
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT owned_account_transaction_list_items_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT owned_account_transaction_list_items_direction_check CHECK (direction IN ('incoming', 'outgoing')),
    CONSTRAINT owned_account_transaction_list_items_kind_check CHECK (
        kind IN ('deposit', 'withdrawal', 'transfer', 'currency_issuance')
    ),
    CONSTRAINT owned_account_transaction_list_items_status_check CHECK (
        status IN ('pending', 'completed', 'failed', 'requires_review')
    )
);

CREATE INDEX IF NOT EXISTS owned_account_transaction_list_items_owner_idx
    ON owned_account_transaction_list_items (owner_type, owner_id);

CREATE INDEX IF NOT EXISTS owned_account_transaction_list_items_owner_occurred_at_idx
    ON owned_account_transaction_list_items (owner_type, owner_id, occurred_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS owned_account_transaction_list_items_account_idx
    ON owned_account_transaction_list_items (account_id);

CREATE INDEX IF NOT EXISTS owned_account_transaction_list_items_currency_idx
    ON owned_account_transaction_list_items (currency_id);

CREATE INDEX IF NOT EXISTS owned_account_transaction_list_items_status_idx
    ON owned_account_transaction_list_items (status);

-- public_account_list_item read model
CREATE TABLE IF NOT EXISTS public_account_list_item_currencies (
    id uuid PRIMARY KEY,
    symbol text NOT NULL,
    name text NOT NULL,
    decimals smallint NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS public_account_list_item_currencies_symbol_idx
    ON public_account_list_item_currencies (symbol);

CREATE TABLE IF NOT EXISTS public_account_list_item_accounts (
    id uuid PRIMARY KEY,
    owner_type text NOT NULL,
    owner_id uuid NOT NULL,
    currency_id uuid NOT NULL,
    status text NOT NULL,
    updated_at timestamptz NOT NULL,
    created_at timestamptz NOT NULL,
    updated_event_sequence bigint NOT NULL,
    CONSTRAINT public_account_list_item_accounts_owner_type_check CHECK (owner_type IN ('user', 'organization')),
    CONSTRAINT public_account_list_item_accounts_status_check CHECK (status IN ('active', 'frozen'))
);

CREATE INDEX IF NOT EXISTS public_account_list_item_accounts_owner_status_created_at_idx
    ON public_account_list_item_accounts (owner_type, owner_id, status, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS public_account_list_item_accounts_owner_status_currency_idx
    ON public_account_list_item_accounts (owner_type, owner_id, status, currency_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS public_account_list_item_accounts_owner_status_id_idx
    ON public_account_list_item_accounts (owner_type, owner_id, status, id);

CREATE INDEX IF NOT EXISTS public_account_list_item_accounts_status_created_at_idx
    ON public_account_list_item_accounts (status, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS public_account_list_item_accounts_status_currency_created_idx
    ON public_account_list_item_accounts (status, currency_id, created_at DESC, id DESC);

CREATE INDEX IF NOT EXISTS public_account_list_item_accounts_status_id_idx
    ON public_account_list_item_accounts (status, id);
