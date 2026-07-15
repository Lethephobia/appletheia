pub struct MintMetadataUpdateInstructionArgs {
    pub mint_id: [u8; 16],
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
