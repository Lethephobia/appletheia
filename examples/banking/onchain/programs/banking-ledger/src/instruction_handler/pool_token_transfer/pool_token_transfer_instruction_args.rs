pub struct PoolTokenTransferInstructionArgs {
    pub idempotency_key: [u8; 16],
    pub mint_id: [u8; 16],
    pub amount: u64,
}
