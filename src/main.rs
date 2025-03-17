mod block;
mod cryptography;
mod policy;
mod state;

mod util {
    pub type Unknown = i32;

    /// represents the time in milliseconds since the *JAM Common Era*, **12:00 UTC January 1, 2025**
    pub type Timeslot = u64;
}

use block::Block;
use policy::Policy;
use state::State;

struct JAM {
    state: State,
    block_level_policy: Policy,
}

fn main() {
    let mut state = state::State::default();
    println!("state: {:?}", state.alpha);
    println!("state: {:?}", state.phi);
    println!("state: {:?}", state.rho);
    println!("state: {:?}", state.beta);
    println!("state: {:?}", state.tau);
    println!("state: {:?}", state.delta);
    println!("state: {:?}", state.chi);
    println!("state: {:?}", state.gamma);
    println!("state: {:?}", state.iota);
    println!("state: {:?}", state.current_validator_set);
    println!("state: {:?}", state.gamma);
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
        transition_function: |state: &mut State, _: &Block| {},
    };

    (policy.transition_function)(&mut state, &block);
}
