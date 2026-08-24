# Ethereum Banking Settlement

This independent Foundry workspace deploys `BankingSettlement`, which coordinates
replay-safe deposits and withdrawals of existing ERC-20 tokens. It does not issue,
mint, burn, or manage the supply of those tokens.

Run `forge test` to build and test the contract. Deploy with
`BANKING_SETTLEMENT_OPERATOR=<address> forge script script/DeployBankingSettlement.s.sol --broadcast`.

Start a local Anvil chain with `script/start-local-chain.sh`. The script defaults to
`127.0.0.1:8545` and chain ID `31337`; override those values with `ETHEREUM_RPC_HOST`,
`ETHEREUM_RPC_PORT`, and `ETHEREUM_CHAIN_ID` when needed.
