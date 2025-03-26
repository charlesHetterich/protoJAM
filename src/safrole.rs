use crate::cryptography::{BLSHash, BandersnatchHash, Ed25519Hash};

struct ValidatorKey {
    /// ([k_b] in paper)
    pub bandersnatch_key: BandersnatchHash,

    /// ([k_e] in paper)
    pub ed25519_key: Ed25519Hash,

    /// ([k_{BLS}] in paper)
    pub bls_key: BLSHash,

    /// ([k_m] in paper)
    pub metadata: [u8; 128],
}

struct Ticket {
    pub identifier: BandersnatchHash,
    pub entry_index: u32,
}

// (γ) in paper
struct SafroleState {
    ///  ([γ_k] in paper)
    pub pending_validator_set: Vec<Ticket>,

    /// isolated Safrole data ([γ_z] in paper)
    pub epoch_root: BandersnatchHash,

    /// isolated Safrole data ([γ_s] in paper)
    pub slot_sealer_series: Vec<Ticket>,

    /// isolated Safrole data ([γ_a] in paper)
    pub ticket_accumulator: Vec<Ticket>,
}
