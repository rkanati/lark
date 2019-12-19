
mod pubtypes;
mod lexing;
mod line_lexer;

pub use pubtypes::*;

use line_lexer::LineLexer;

pub struct Tokens<'a>(Box<dyn Iterator<Item = LexResult<'a>> + 'a>);

impl<'a> Tokens<'a> {
    pub fn new(src: &'a str) -> Tokens<'a> {
        let inner = src.lines()
            .enumerate()
            .flat_map(|(index, line)| LineLexer::new(line, index+1));
        Tokens(Box::new(inner))
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = LexResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub fn tokens<'a> (src: &'a str) -> Tokens<'a> {
    Tokens::new(src)
}

pub fn lex<'a> (src: &'a str) -> Result<Vec<Token<'a>>, LexError> {
    tokens(src).collect()
}

