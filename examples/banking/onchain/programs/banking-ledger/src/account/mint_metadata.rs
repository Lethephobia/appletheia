pub struct MintMetadata;

impl MintMetadata {
    pub const SEED: &[u8] = b"mint_metadata";
    pub const SPACE: usize = 1024;
    pub const MAX_NAME_BYTES: usize = 64;
    pub const MAX_SYMBOL_BYTES: usize = 16;
    pub const MAX_URI_BYTES: usize = 256;
}
