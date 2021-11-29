use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokenizer {
    state: State,
    input: String,
    idx: usize,
    current_token: Option<Token>,
    token_buffers: VecDeque<Token>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    StartTag,
    EndTag,
    Character,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    kind: TokenKind,
    self_closing: bool,

    tag_name: String,
    val: char,
}

pub struct TokenOpt {
    kind: TokenKind,
    tag_name: String,
    self_closing: bool,
    val: char,
}

impl Token {
    pub fn new(opt: TokenOpt) -> Self {
        Token {
            kind: opt.kind,
            self_closing: opt.self_closing,

            tag_name: opt.tag_name.to_string(),
            val: opt.val,
        }
    }

    pub fn append_tag_name(&mut self, tag_name: String) {
        self.tag_name.push_str(&tag_name);
    }

    pub fn set_self_closing(&mut self) {
        self.self_closing = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
    DataState,
    TagOpenState,
    TagNameState,
    SelfClosingStartTagState,
    EndTagOpenState,
}

macro_rules! go {
    ($x:expr, $y:expr) => {
        $x.state = $y;
    };
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        dbg!(input);
        let tok = Self {
            state: State::DataState,
            input: input.to_string(),
            idx: 0,
            current_token: None,
            token_buffers: VecDeque::new(),
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
                    let tok = Token::new(TokenOpt {
                        kind: TokenKind::Character,
                        tag_name: String::new(),
                        self_closing: false,
                        val: c,
                    });
                    self.current_token = Some(tok);
                    self.emit_current_token();
                }
            },
            State::TagOpenState => match c {
                'a'..='z' | 'A'..='Z' => {
                    let tok = Token::new(TokenOpt {
                        kind: TokenKind::StartTag,
                        tag_name: "".to_string(),
                        self_closing: false,
                        val: '\0',
                    });
                    self.current_token = Some(tok);
                    self.reconsume(State::TagNameState)
                }
                '/' => {
                    go!(self, State::EndTagOpenState);
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
                        .append_tag_name(c.to_ascii_lowercase().to_string());
                }
                '/' => {
                    println!("todo TagNameState /");
                    go!(self, State::SelfClosingStartTagState);
                }
                '>' => {
                    let tok = self.current_token.clone().unwrap();
                    self.emit_current_token();
                    go!(self, State::DataState);
                }
                _ => {
                    unimplemented!("TagNameState {}", c)
                }
            },
            State::SelfClosingStartTagState => match c {
                '>' => {
                    println!("emit current tag");
                    self.current_token.as_mut().unwrap().set_self_closing();
                    self.emit_current_token();
                    go!(self, State::DataState);
                }
                _ => {
                    unimplemented!("SelfClosingStartTagState {}", c)
                }
            },
            State::EndTagOpenState => match c {
                'a'..='z' | 'A'..='Z' => {
                    println!("todo EndTagOpenState {}", c);
                    let tok = Token::new(TokenOpt {
                        kind: TokenKind::EndTag,
                        tag_name: "".to_string(),
                        self_closing: false,
                        val: '\0',
                    });
                    self.current_token = Some(tok);
                    self.reconsume(State::TagNameState);
                }
                _ => {
                    unimplemented!("EndTagOpenState {}", c)
                }
            },
            _ => {
                return false;
            }
        }
        self.idx += 1;
        true
    }

    fn reconsume(&mut self, state: State) {
        self.idx -= 1;
        go!(self, state);
    }

    pub fn at_eof(&self) -> bool {
        self.idx >= self.input.len() as usize
    }

    pub fn emit(&mut self, token: &Token) {
        self.token_buffers.push_back(token.clone());
    }

    pub fn emit_current_token(&mut self) {
        self.emit(&self.current_token.clone().unwrap());
    }

    pub fn take_next_token(&mut self) -> Option<Token> {
        self.token_buffers.pop_front()
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
        let str = "<html>hi</html>";
        let expected = vec![
            Token::new(TokenOpt {
                kind: TokenKind::StartTag,
                tag_name: "html".to_string(),
                self_closing: false,
                val: '\0',
            }),
            Token::new(TokenOpt {
                kind: TokenKind::Character,
                tag_name: String::new(),
                self_closing: false,
                val: 'h',
            }),
            Token::new(TokenOpt {
                kind: TokenKind::Character,
                tag_name: String::new(),
                self_closing: false,
                val: 'i',
            }),
            Token::new(TokenOpt {
                kind: TokenKind::EndTag,
                tag_name: "html".to_string(),
                self_closing: false,
                val: '\0',
            }),
        ];
        let mut tokenizer = Tokenizer::new(str);
        while !tokenizer.at_eof() {
            assert_eq!(tokenizer.consume(), true);
        }
        assert_eq!(tokenizer.token_buffers.len(), expected.len());
        let mut i = 0;
        while !tokenizer.token_buffers.is_empty() {
            let tok = tokenizer.take_next_token().unwrap();
            assert_eq!(expected[i], tok);
            i += 1;
        }
        assert_eq!(tokenizer.token_buffers.len(), 0);
    }
}
