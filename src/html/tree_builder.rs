#[derive(Debug, Clone, Copy)]
pub struct TreeBuilder {
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    Initial,
}

impl TreeBuilder {
    pub fn new() -> TreeBuilder {
        TreeBuilder {
            state: State::Initial,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_state_is_initial() {
        let tree_builder = TreeBuilder::new();
        assert_eq!(tree_builder.state, State::Initial);
    }
}
