// update for Bandersnatch (144-bytes), Ed25519 (144-bytes), BLS (144-bytes)
pub type Octet = u8;
pub type BandersnatchHash = [Octet; 32];
pub type Ed25519Hash = [Octet; 32];
pub type BLSHash = [Octet; 144];

pub type Signature = i32;
