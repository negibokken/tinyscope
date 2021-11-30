use std::collections::VecDeque;

use crate::html::token::*;
#[derive(Debug, Clone)]
pub struct TreeBuilder {
    state: State,
    tokens: VecDeque<Token>,

    document: Node,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            children: Vec::new(),
        }
    }
    pub fn apend_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    Initial,
    BeforeHtml,
}

fn take_next_token(tokens: &mut VecDeque<Token>) -> Option<Token> {
    if tokens.len() > 0 {
        Some(tokens.pop_front().unwrap())
    } else {
        None
    }
}

impl TreeBuilder {
    pub fn new(tokens: &mut VecDeque<Token>) -> TreeBuilder {
        TreeBuilder {
            state: State::Initial,
            tokens: tokens.clone(),

            document: Node { children: vec![] },
        }
    }

    pub fn reprocess_token(&mut self, token: Token) {
        self.tokens.push_front(token);
    }

    pub fn consume(&mut self) -> bool {
        if self.tokens.len() == 0 {
            return false;
        }
        match self.state {
            State::Initial => match take_next_token(&mut self.tokens) {
                Some(tok) => match tok.kind {
                    _ => {
                        self.state = State::BeforeHtml;
                        self.reprocess_token(tok);
                    }
                },
                None => {
                    return false;
                }
            },
            State::BeforeHtml => match take_next_token(&mut self.tokens) {
                Some(tok) => match tok.kind {
                    TokenKind::StartTag if tok.tag_name == "html" => {
                        let child = Node::new();
                        self.document.apend_child(child);
                    }
                    _ => {
                        self.state = State::BeforeHtml;
                    }
                },
                None => {
                    return false;
                }
            },
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initial_state_is_initial() {
        let tree_builder = TreeBuilder::new(&mut VecDeque::new());
        assert_eq!(tree_builder.state, State::Initial);
        assert_eq!(tree_builder.document, Node { children: vec![] });
    }

    #[test]
    fn consume() {
        let mut vd = VecDeque::new();
        vd.push_back(Token::new(TokenOpt {
            kind: TokenKind::StartTag,
            tag_name: "html".to_string(),
            self_closing: false,
            val: '\0',
        }));
        let mut tree_builder = TreeBuilder::new(&mut vd);
        while tree_builder.consume() {}
        assert_eq!(tree_builder.state, State::BeforeHtml);
        assert_eq!(
            tree_builder.document,
            Node {
                children: vec![Node { children: vec![] }]
            }
        );
    }
}
