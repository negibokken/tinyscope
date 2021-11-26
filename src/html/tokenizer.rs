#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokenizer {
    state: State,
    input: String,
    idx: usize,
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

    fn consume(&mut self) -> bool {
        if self.idx >= self.input.len() as usize {
            return false;
        }
        println!("{}", self.input.chars().nth(self.idx).unwrap());
        self.idx += 1;
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
        let str = "hello";
        let mut tokenizer = Tokenizer::new(str);
        let mut idx = 0;
        while idx < str.len() as usize {
            idx += 1;
            assert_eq!(tokenizer.consume(), true)
        }
    }
}
