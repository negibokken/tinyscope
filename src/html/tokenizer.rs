#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokenizer {
    state: State,
    input: String,
    idx: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    DataState,
    TagOpenState,
    TagNameState,
}

fn ascii_letter(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
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
        println!("> {}", self.input.chars().nth(self.idx).unwrap());
        let c = self.input.chars().nth(self.idx).unwrap();
        match self.state {
            State::DataState => match c {
                '<' => {
                    self.state = State::TagOpenState;
                }
                _ => {
                    return false;
                }
            },
            State::TagOpenState => match c {
                'a'..='z' | 'A'..='Z' => {
                    self.state = State::TagNameState;
                    self.idx -= 1;
                }
                _ => {
                    unimplemented!("TagOpenState {}", c)
                }
            },
            State::TagNameState => match c {
                'a'..='z' | 'A'..='Z' => {
                    println!("todo TagNameState {}", c);
                }
                _ => {
                    unimplemented!("TagNameState {}", c)
                }
            },
            _ => {
                return false;
            }
        }
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
        let str = "<html/>";
        let mut tokenizer = Tokenizer::new(str);
        let mut idx = 0;
        while idx < str.len() as usize {
            idx += 1;
            assert_eq!(tokenizer.consume(), true)
        }
    }
}
