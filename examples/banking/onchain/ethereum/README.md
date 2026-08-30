# Ethereum Banking Settlement

This independent Foundry workspace deploys `BankingSettlement`, which coordinates
replay-safe deposits and withdrawals of existing ERC-20 tokens. A token binding only
requires an ERC-20 contract usable for exact settlement; ERC-2612 and ERC-3009 are
optional deposit authorization mechanisms. The settlement contract does not issue,
mint, burn, or manage the supply of those tokens. ERC-20 transfers use OpenZeppelin
`SafeERC20`, and settlement can be paused. Withdrawals are submitted by an account
with the shared operator role. For deposits, an operator signs an expiring EIP-712
authorization. The token owner may use a pre-existing allowance, sign an ERC-2612
permit that executes with `transferFrom`, or sign an ERC-3009 receive authorization
that transfers without creating an allowance.

Gas-sponsored deposits use ERC-4337 EntryPoint v0.9 and an EIP-7702 delegated
`BankingSettlementAccount`. `BankingSettlementPaymaster` accepts only a signed
sponsorship for a single `settleDeposit` call to the configured settlement proxy from
that account implementation. A bundler submits the UserOperation and the paymaster's
EntryPoint deposit pays its gas. When ERC-2612 or ERC-3009 is selected, the token
owner signs both the token authorization and the UserOperation; the sponsorship signer
independently controls which operation the paymaster accepts. Operator
authorizations support EOAs and ERC-1271 contract wallets. Repeating the same
settlement ID with the same values succeeds without another transfer; conflicting
values are rejected. The operator role is managed by a delayed default administrator.
The implementation
uses an OpenZeppelin UUPS proxy, ERC-7201 namespaced storage, and a dedicated upgrader
role. Tests and deployment scripts use OpenZeppelin Foundry Upgrades validations.

Initialize the pinned dependencies after cloning the repository with
`forge soldeer install`, then run `forge test` to build and test the contract.

Generate the committed settlement and token-interface ABIs for server-side Alloy
bindings with `script/generate-abi.sh`. The generated files are written to `abi/`.

Deploy with `BANKING_SETTLEMENT_ADMIN=<address>`,
`BANKING_SETTLEMENT_PAUSER=<address>`,
`BANKING_SETTLEMENT_OPERATOR=<address>`, and
`BANKING_SETTLEMENT_UPGRADER=<address>` set, using
`forge clean && forge script script/DeployBankingSettlement.s.sol --broadcast`.

Upgrade an existing proxy with `BANKING_SETTLEMENT_PROXY=<address>` and
`BANKING_SETTLEMENT_IMPLEMENTATION_CONTRACT=<artifact>` set, using
`forge clean && forge script script/UpgradeBankingSettlement.s.sol --broadcast --sender <upgrader>`.
Use a multisig or timelock contract as the production upgrader rather than an
individual externally owned account.

Deploy the stateless EIP-7702 account implementation and paymaster after the
settlement proxy with `BANKING_SETTLEMENT_PROXY=<address>`,
`BANKING_PAYMASTER_ADMIN=<address>`,
`BANKING_PAYMASTER_FUND_MANAGER=<address>`,
`BANKING_PAYMASTER_SIGNER_MANAGER=<address>`, and
`BANKING_PAYMASTER_SPONSORSHIP_SIGNER=<address>` set, using
`forge script script/DeployBankingSettlementAccountAbstraction.s.sol --broadcast`.
Fund the paymaster's EntryPoint balance through `deposit`. In production, use a
multisig or timelock for the paymaster administration and fund-management roles, and
keep the sponsorship signer in the backend's key-management system.
The paymaster is deliberately not upgradeable: changing its sponsorship policy
requires deploying and funding a new paymaster, then retiring the old one through its
fund manager.

Start a local Anvil chain with `script/start-local-chain.sh`. The script defaults to
`127.0.0.1:8545` and chain ID `31337`; override those values with `ETHEREUM_RPC_HOST`,
`ETHEREUM_RPC_PORT`, and `ETHEREUM_CHAIN_ID` when needed.
