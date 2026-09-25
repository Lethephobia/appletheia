use alloy::sol;

sol!(
    #[sol(rpc)]
    BankingSettlementAccount,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../onchain/ethereum/abi/BankingSettlementAccount.json"
    )
);
