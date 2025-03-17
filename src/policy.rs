use crate::block::Block;
use crate::state::State;

pub struct Policy {
    /// [Υ] upsilon in paper
    pub transition_function: fn(state: &mut State, block: &Block),
}
