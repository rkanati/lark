
#[derive(Clone, Copy, Debug)]
pub struct SrcLoc {
    pub line:   usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Ident,
    LitInt,
    LitString,
    Punct,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    kind:     TokenKind,
    line_num: usize,
    leader:   &'a str,
    spelling: &'a str,
}

impl<'a> Token<'a> {
    pub(super) fn new(kind: TokenKind, line_num: usize, leader: &'a str, spelling: &'a str)
        -> Token<'a>
    {
        Token { kind, line_num, leader, spelling }
    }

    fn column(&self) -> usize {
        // TODO: compute column from leader
        self.leader.len() + 1
    }

    pub fn location(&self) -> SrcLoc {
        SrcLoc { line: self.line_num, column: self.column() }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn spelling(&self) -> &'a str {
        self.spelling
    }

    pub fn expect(self, kind: TokenKind) -> Option<Self> {
        if kind == self.kind { Some(self) }
        else                 { None }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum LexError {
    BadCharacter(char),
    UnclosedString(char)
}

pub type LexResult<'a> = Result<Token<'a>, LexError>;

