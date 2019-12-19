
use crate::lexer::Token;

//
// file = toplevel*
// toplevel = function
// function = 'fn' ident '(' comma-list[ident]? ')' block
//
// block = 'do' statement* 'end'
//
// statement = let-binding
//           | return-statement
// let-binding = 'let' ident '=' expression
// return-statement = 'return' expression
//
// expression = literal
//            | infix-expression
//            | application
//            | paren-expression
// application = ident '(' comma-list[expression]? ')'
//
// comma-list[rule] = (rule ',')* rule (',')?
//

pub enum Expr<'a> {
    Call(Ident<'a>, Vec<OwnExpr<'a>>),
    Name(Ident<'a>),
    LitInt(Token<'a>),
    LitString(Token<'a>)
}

type OwnExpr<'a> = Box<Expr<'a>>;

pub enum Statement<'a> {
    LetBinding(LetBinding<'a>),
    Return(Expr<'a>)
}

pub struct Block<'a> (Vec<Statement<'a>>);

pub struct Ident<'a> (Token<'a>);

pub struct Argument<'a> (Token<'a> );

pub struct Function<'a> {
    name:      Ident<'a>,
    arguments: Vec<Argument<'a>>,
    body:      Block<'a>
}

impl<'a> Function<'a> {
    fn new(name: Ident<'a>, arguments: Vec<Argument<'a>>, body: Block<'a>) -> Function<'a> {
        Function { name, arguments, body }
    }
}

pub struct LetBinding<'a> {
    name:  Ident<'a>,
    value: Expr<'a>
}

pub enum TopLevel<'a> {
    Function(Function<'a>),
    LetBinding(LetBinding<'a>)
}

pub struct File<'a> (Vec<TopLevel<'a>>);

impl<'a> File<'a> {
    pub fn new(toplevels: Vec<TopLevel>) -> File {
        File(toplevels)
    }
}

mod parser {
    use {
        super::*,
        crate::{
            parse::{Parse, ParseResult, Parser, ParseError},
            lexer::{Token, TokenKind}
        },
    };

    pub trait TokenIteratorExt<'a> : Iterator<Item = Token<'a>> + Sized {
        fn expect(mut self, kind: TokenKind, spelling: &'static str) -> Result<Self, ParseError> {
            let token = self.next().ok_or(ParseError::ParseError)?;
            if token.kind() == kind && token.spelling() == spelling {
                Ok(self)
            }
            else {
                Err(ParseError::ParseError)
            }
        }
    }

    impl<'a, I> TokenIteratorExt<'a> for I
        where I: Iterator<Item = Token<'a>> + Sized { }

    impl<'a> Parse for LetBinding<'a> {
        fn parse<Iter> (tokens: Iter) -> ParseResult<Iter, Self> {
            let tokens = tokens.expect(TK::Ident, "let")?;
            let (name, tokens) = Ident::parse(tokens)?;
            let tokens = tokens.expect(TK::Punct, "=")?;
            let (value, tokens) = Expr::parse(tokens)?;
            Ok((LetBinding { name, value }, tokens))
        }
    }

    impl<'a> Parse for Function<'a> {
        fn parse<Iter> (tokens: Iter) -> ParseResult<Iter, Self> {
            let (name, tokens) = Ident::parse(tokens)?;
            let (arguments, tokens) = Argument::parser()
                .list(punct(','))
                .parse(tokens)?;
            let (body, tokens) = Block::parse(tokens)?;
            Ok((Function { name, arguments, body }, tokens))
        }
    }

    impl<'a> Parse for TopLevel<'a> {
        fn parse<Iter> (tokens: Iter) -> ParseResult<Iter, Self> {
            Function::parser()
                .map(|f| TopLevel::Function(f))
                .or_else(
                    LetBinding::parser()
                    .map(|b| TopLevel::LetBinding(b))
                )
                .parse(tokens)
        }
    }

    impl<'a> Parse for File<'a> {
        fn parse<Iter> (tokens: Iter) -> ParseResult<Iter, Self> {
            let (toplevels, tokens) = Vec::parse(tokens)?;
            Ok((File::new(toplevels), tokens))
        }
    }
}

