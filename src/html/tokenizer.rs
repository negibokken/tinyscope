#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokenizer {
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    DataState,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        dbg!(input);
        if cfg!(debug_assertions) {
            println!(
                "{:?}",
                Tokenizer {
                    state: State::DataState,
                },
            );
        }
        Tokenizer {
            state: State::DataState,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_data() {
        let tokenizer = Tokenizer::new("hello".to_string());
        assert_eq!(tokenizer.state, State::DataState);
    }
}
