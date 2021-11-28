#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokenizer {
    state: State,
    input: String,
    idx: usize,
    current_token: Option<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Token {
    tag_name: String,
}

impl Token {
    pub fn new(tag_name: &str) -> Self {
        Token {
            tag_name: tag_name.to_string(),
        }
    }
    pub fn append_tag_name(&mut self, tag_name: String) {
        self.tag_name.push_str(&tag_name);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    DataState,
    TagOpenState,
    TagNameState,
    SelfClosingStartTagState,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        dbg!(input);
        let tok = Self {
            state: State::DataState,
            input: input.to_string(),
            idx: 0,
            current_token: None,
        };
        if cfg!(debug_assertions) {
            println!("{:?}", tok,);
        }
        tok
    }

    fn consume(&mut self) -> bool {
        println!("idx: {}", self.idx);
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
                    let tok = Token::new("");
                    self.current_token = Some(tok);
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
                    self.current_token
                        .as_mut()
                        .unwrap()
                        .append_tag_name(c.to_string());
                }
                '/' => {
                    println!("todo TagNameState /");
                    self.state = State::SelfClosingStartTagState;
                }
                _ => {
                    unimplemented!("TagNameState {}", c)
                }
            },
            State::SelfClosingStartTagState => match c {
                '>' => {
                    println!("emit current tag");
                    self.emit(self.current_token.as_ref().unwrap());
                    self.state = State::DataState;
                }
                _ => {
                    unimplemented!("SelfClosingStartTagState {}", c)
                }
            },
            _ => {
                return false;
            }
        }
        self.idx += 1;
        true
    }

    pub fn at_eof(&self) -> bool {
        self.idx >= self.input.len() as usize
    }

    pub fn emit(&self, token: &Token) {
        println!("emit {:?}", token);
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
        while !tokenizer.at_eof() {
            assert_eq!(tokenizer.consume(), true);
        }
        println!("{:?}", tokenizer.current_token);
    }
}
