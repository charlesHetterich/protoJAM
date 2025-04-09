mod block;
mod cryptography;
mod encoder_decoder;
mod generics;
mod policy;
mod safrole;
mod state;

mod util {
    pub type Unknown = i32;

    /// represents the time in milliseconds since the *JAM Common Era*, **12:00 UTC January 1, 2025**
    pub type Timeslot = u64;
}

use block::Block;
use encoder_decoder::{Decode, Encode};
use generics::Config;
use policy::Policy;
use state::State;

struct JAM<T: Config> {
    state: State<T>,
    block_level_policy: Policy<T>,
}

#[derive(Debug, Default)]
struct MockConfig;

impl Encode for u8 {
    fn encode(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Decode for u8 {
    fn decode(data: &[u8]) -> Self {
        data[0]
    }
}

impl Config for MockConfig {
    type Ticket = u8;
    type Preimage = u8;
    type Report = u8;
    type Assurance = u8;
    type Dispute = u8;
}

fn main() {
    let mut state = State::<MockConfig>::default();
    println!("state: {:?}", state.alpha);
    println!("state: {:?}", state.phi);
    println!("state: {:?}", state.rho);
    println!("state: {:?}", state.beta);
    println!("state: {:?}", state.tau);
    println!("state: {:?}", state.delta);
    println!("state: {:?}", state.chi);
    println!("state: {:?}", state.safrole_core_state);
    println!("state: {:?}", state.iota);
    println!("state: {:?}", state.active_validator_set);
    println!("state: {:?}", state.lambda);
    println!("state: {:?}", state.eta);
    println!("state: {:?}", state.theta);
    println!("state: {:?}", state.xi);
    println!("state: {:?}", state.psi);
    println!("state, {:?}!", state.pi);

    let block = block::Block {
        header: block::Header::default(),
        extrinsics: block::Extrinsics::default(),
    };
    println!("Hello, {:?}!", block.header);
    println!("Hello, {:?}!", block.extrinsics);

    let policy = policy::Policy {
        transition_function: |state: &mut State<MockConfig>, _: &Block<MockConfig>| {},
    };

    (policy.transition_function)(&mut state, &block);
}

// More resources from (https://polkadot-blockchain-academy.github.io/pba-content/current/syllabus/6-Polkadot/14-jam-math-to-code-slides.html#/11)
// let timeslot_ = get_timeslot(h);
// let history_daga = get_history_data(h, &state.recent_history);
// let history_ = get_history_(h, &ext.tickets, &history_daga, c);
