#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    StartTag,
    EndTag,
    Character,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub self_closing: bool,

    pub tag_name: String,
    pub val: char,
}

pub struct TokenOpt {
    pub kind: TokenKind,
    pub tag_name: String,
    pub self_closing: bool,
    pub val: char,
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
