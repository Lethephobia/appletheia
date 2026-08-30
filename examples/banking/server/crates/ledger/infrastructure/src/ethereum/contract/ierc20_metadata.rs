use alloy::sol;

sol!(
    #[sol(rpc)]
    IERC20Metadata,
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../onchain/ethereum/abi/IERC20Metadata.json"
    )
);
