use alloy::sol;

sol!(
    #[sol(rpc)]
    BankingSettlementPaymaster,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../onchain/ethereum/abi/BankingSettlementPaymaster.json"
    )
);
