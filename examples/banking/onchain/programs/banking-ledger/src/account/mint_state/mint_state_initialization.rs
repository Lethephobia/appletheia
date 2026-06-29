pub struct MintStateInitialization {
    pub mint_id: [u8; 16],
    pub bump: u8,
    pub mint_bump: u8,
    pub mint_metadata_bump: u8,
    pub program_authority_bump: u8,
}
