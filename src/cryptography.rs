// update for Bandersnatch (144-bytes), Ed25519 (144-bytes), BLS (144-bytes)
pub type Octet = u8;

pub trait Hash {}

pub type BandersnatchHash = [Octet; 32];
impl Hash for BandersnatchHash {}

pub type Ed25519Hash = [Octet; 32];
// impl Hash for Ed25519Hash {}

pub type BLSHash = [Octet; 144];
impl Hash for BLSHash {}

pub type Signature = i32;
