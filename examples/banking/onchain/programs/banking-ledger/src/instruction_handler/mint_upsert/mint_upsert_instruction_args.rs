pub struct MintUpsertInstructionArgs {
    pub mint_id: [u8; 16],
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
