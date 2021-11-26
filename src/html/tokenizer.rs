#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokenizer {
    state: State,
    input: String,
    idx: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    DataState,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        dbg!(input);
        let tok = Self {
            state: State::DataState,
            input: input.to_string(),
            idx: 0,
        };
        if cfg!(debug_assertions) {
            println!("{:?}", tok,);
        }
        tok
    }

    fn consume(&self) -> bool {
        println!("consume");
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_data() {
        let tokenizer = Tokenizer::new("hello");
        assert_eq!(tokenizer.state, State::DataState);
    }

    #[test]
    fn consume() {
        let tokenizer = Tokenizer::new("hello");
        assert_eq!(tokenizer.consume(), true)
    }
}
