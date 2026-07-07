use anchor_lang::prelude::*;

pub mod mint_state_initialization;

pub use mint_state_initialization::MintStateInitialization;

#[account]
pub struct MintState {
    pub version: u8,
    pub bump: u8,
    pub mint_bump: u8,
    pub mint_metadata_bump: u8,
    pub program_authority_bump: u8,
}

impl MintState {
    pub const SEED: &[u8] = b"mint_state";
    pub const VERSION: u8 = 1;
    pub const LEN: usize = 1 + 1 + 1 + 1 + 1;

    pub fn initialize(&mut self, initialization: MintStateInitialization) {
        let MintStateInitialization {
            bump,
            mint_bump,
            mint_metadata_bump,
            program_authority_bump,
        } = initialization;

        self.version = Self::VERSION;
        self.bump = bump;
        self.mint_bump = mint_bump;
        self.mint_metadata_bump = mint_metadata_bump;
        self.program_authority_bump = program_authority_bump;
    }

    pub fn is_initialized(&self) -> bool {
        self.version != 0
    }
}
