use crate::block::Block;

/// The main shared/synchronised state of the overall JAM system ([σ] sigma in paper).
/// TODO : figure out actual types of each of these components
#[derive(Debug, Default)]
pub struct State {
    ////// CORES DATA
    //////////////////////////////////////////////////////
    /// authorization requirement which work done
    /// on this core must satisfy ([α] alpha in paper)
    pub alpha: i32,

    /// queue of work packages (?) ([φ] phi in paper)
    pub phi: i32,

    /// each core's currently asigned *report*, the availability of who's *work-package*
    /// must yet be assured by a super-majority of validators ([ρ] rho in paper)
    pub rho: i32,

    ////// BLOCK DATA
    //////////////////////////////////////////////////////
    /// recent block data ([β] beta in paper)
    /// is this where we store 24-hour history of ancestor headers, $\bold{A}$?
    pub beta: Vec<Block>,

    /// recent timeslot index ([τ] tau in paper)
    pub tau: i32,

    //// SEVICE DATA
    //////////////////////////////////////////////////////
    /// portion of state dealing with services ([δ] delta in paper)
    pub delta: i32,

    /// list of identities of privilaged services ([χ] chi in paper)
    pub chi: i32,

    ////// SAFROLE / VALIDATOR SET DATA
    //////////////////////////////////////////////////////
    /// isolated Safrole data ([γ] gamma in paper)
    pub gamma: i32,

    /// queue of future best validators ([ι] iota in paper)
    pub iota: i32,

    /// set of best (current) validators ([κ] kappa in paper)
    pub current_validator_set: i32,

    /// set of archived best validators ([λ] lambda in paper)
    pub lambda: i32,

    ////// MISC //////
    //////////////////////////////////////////////////////
    /// on-chain entropy pool ([η] eta in paper)
    pub eta: i32,

    /// work-reports ready for accumulation step ([θ] theta in paper)
    pub theta: i32,

    /// recently accumulated work-packages ([ξ] xi in paper)
    pub xi: i32,

    /// judgements ([ψ] psi in paper)
    pub psi: i32,

    /// validator statistics ([π] pi in paper)
    pub pi: i32,
}
