use crate::cryptography::{BandersnatchHash, Hash, Signature};
use crate::generics::Config;
use crate::util::{Timeslot, Unknown};

// TODO! figure out proper hash type for each field

#[derive(Debug, Default)]
pub struct Header {
    /// ([H_p] in paper)
    pub parent_hash: BandersnatchHash,

    /// root hash of a Merkle trie composed by the mapping of the *prior* state's Merkle root ([H_r] in paper)
    pub parent_state_root: BandersnatchHash,

    /// a Merkle commitment to the block's extrinsic data ([H_x] in paper)
    pub extrinsic_hash: BandersnatchHash,

    /// time at which the block was published (?) ([H_t] in paper)
    pub timeslot_index: Timeslot,

    /// ([H_e] in paper)
    pub epoch: i32,

    /// ([H_w] in paper)
    pub winning_tickets: Unknown,

    /// ([H_o] in paper)
    pub offenders: Unknown,

    /// public key to identify the author of the block We use this as an index into the posterior current validator set, [κ] kappa ([H_i] in paper)
    pub bandersnatch_block_author_index: Unknown,

    /// ([H_v] in paper)
    pub vrf_signature: Signature,

    /// ([H_s] in paper)
    pub block_seal: Signature,
}

#[derive(Debug, Default)]
pub struct Extrinsics<T: Config> {
    /// used to select next validator as block author (E_T in paper)
    pub tickets: Vec<T::Ticket>,

    /// static data being requested to be available for workloads to fetch readily (E_P in paper)
    pub pre_images: Vec<T::Preimage>,

    /// newly completed workloads that have been validated by specific validators (E_G in paper)
    pub reports: Vec<T::Report>,

    /// assurance from validators as to which workloads' input data they have correctly
    /// recieved and are storing locally (E_A in paper)
    pub availability: Vec<T::Assurance>,

    /// information on disputes between validators over the validity of reports (E_D in paper)
    pub disputes: Vec<T::Dispute>,
}

/// Extrinsic as given by (https://polkadot-blockchain-academy.github.io/pba-content/current/syllabus/6-Polkadot/14-jam-math-to-code-slides.html#/8)
// struct __Extrinsic {
//     tickets: Vec<Ticket>,
//     disputes: Disputes,
//     preimages: Vec<Preimage>,
//     assurances: Vec<Assurance>,
//     guarantees: Vec<Guarantee>,
// }

#[derive(Debug, Default)]
pub struct Block<T: Config> {
    pub header: Header,
    pub extrinsics: Extrinsics<T>,
}
