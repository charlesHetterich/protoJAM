#[derive(Debug)]
pub struct Header {}

#[derive(Debug)]
pub struct Extrinsics {}

#[derive(Debug)]
pub struct Block {
    pub header: Header,
    pub extrinsics: Extrinsics,
}
