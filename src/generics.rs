use crate::encoder_decoder::{Decode, Encode};
use std::fmt::Debug;

pub trait Config {
    type Ticket: Encode + Decode + Debug + Default;
    type Preimage: Encode + Decode + Debug + Default;
    type Report: Encode + Decode + Debug + Default;
    type Assurance: Encode + Decode + Debug + Default;
    type Dispute: Encode + Decode + Debug + Default;
}
