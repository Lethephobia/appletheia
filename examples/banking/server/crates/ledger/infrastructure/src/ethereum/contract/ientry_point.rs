use alloy::sol;

sol!(
    #[sol(rpc)]
    IEntryPoint,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../onchain/ethereum/abi/IEntryPoint.json"
    )
);
