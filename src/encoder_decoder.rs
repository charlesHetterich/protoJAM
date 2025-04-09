pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decode {
    fn decode(data: &[u8]) -> Self;
}
