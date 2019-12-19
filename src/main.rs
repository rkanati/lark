
mod lexer;
mod parse;
mod syn;

use {
    crate::{
        lexer::{Token, TokenKind as TK, lex},
    },
};


fn main() {
    let src = include_str!("../test.lark");
    let tokens = lex(src).unwrap();
    let syn = syn::File::parse(tokens.iter().copied()).unwrap();
}

