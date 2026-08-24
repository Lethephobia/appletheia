# Banking example

The Ledger keeps fiat-denominated account balances separate from external token settlement.
Each Account stores an immutable `CurrencyId`; the event-sourced `Currency` aggregate owns its
`CurrencyCode`, `CurrencyDecimals`, lifecycle, and any number of independently identified
`TokenBinding` entities. `CurrencyAmount` contains only a checked `u128` quantity in the
Currency's smallest unit and never uses floating-point arithmetic.

Currency and TokenBinding events are projected into PostgreSQL-backed Read Models. Account and
transaction queries join those projections for display metadata. Internal transfers compare only
the two Account Currency IDs and remain independent from Currency activation, token bindings, and
blockchain availability.

Deposit and Withdrawal commands select an active `TokenBindingId`, then pin its `ChainNetwork` and
`TokenAddress` in the operation. Successful settlement records a validated Solana transaction
signature or Ethereum transaction hash. Wallet bookmarks store a chain-explicit
`TokenOwnerAddress` and remain independent from a Currency or token.

## On-chain workspaces

- `onchain/solana` is the independent Anchor workspace for the `banking-settlement` program. It
  transfers existing SPL Token or Token-2022 assets and records idempotent receipts keyed by
  Deposit ID or Withdrawal ID.
- `onchain/ethereum` is the independent Foundry workspace for the `BankingSettlement` contract.
  It coordinates existing ERC-20 transfers and rejects repeated settlement IDs.

Neither implementation creates tokens, publishes token metadata, or manages token supply. Token
existence, interface support, decimals, and settlement usability are checked by chain-specific
binding-admission and settlement infrastructure rather than persisted as mutable domain facts.

## Breaking local reset

Issue #128 intentionally changes event payloads, snapshots, projections, database rows, and the
on-chain layout. Existing Banking example data is not upcast. Recreate the local PostgreSQL
database and rerun every migration, rebuild and redeploy the Solana and Ethereum settlement
programs, and regenerate any Anchor, Solidity, or API clients. Do not reuse old Account, Currency,
CurrencyIssuance, Deposit, or Withdrawal streams.
