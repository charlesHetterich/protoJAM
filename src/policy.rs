use crate::block::Block;
use crate::generics::Config;
use crate::state::State;

pub struct Policy<T: Config> {
    /// [Υ] upsilon in paper
    pub transition_function: fn(state: &mut State<T>, block: &Block<T>),
}
