use alloy::sol;

sol!(
    #[sol(rpc)]
    BankingSettlement,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../onchain/ethereum/abi/BankingSettlement.json"
    )
);
