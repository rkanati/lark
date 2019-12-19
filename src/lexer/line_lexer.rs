
use super::{
    lexing::Lexing,
    Token, TokenKind,
    LexResult, LexError,
};

pub struct LineLexer<'a> {
    lexing: Lexing<'a>,
    line: usize
}

impl<'a> LineLexer<'a> {
    pub fn new(slice: &'a str, line: usize) -> LineLexer<'a> {
        let lexing = Lexing::new(slice);
        LineLexer { lexing, line }
    }

    fn done(&mut self, kind: TokenKind, lexing: Lexing<'a>) -> Token<'a> {
        let (mat, rest) = lexing.consume();
        self.lexing = rest;
        Token::new(kind, self.line, mat.leader, mat.spelling)
    }
}

impl<'a> Iterator for LineLexer<'a> {
    type Item = LexResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(skipped) = self.lexing.take_while(char::is_whitespace) {
            self.lexing = skipped.discard();
        }

        let (ch, lexing) = self.lexing.get()?;

        let token = match ch {
            _ if is_ident_initial(ch) => {
                let lexing = lexing.take_while(is_ident_char)?;
                self.done(TokenKind::Ident, lexing)
            }

            _ if is_digit(ch) => {
                let lexing = lexing.take_while(is_digit)?;
                self.done(TokenKind::LitInt, lexing)
            }

            _ if is_quote(ch) => {
                let open_quote = ch;
                let mut lexing_string = lexing;

                let mut string_loop = || loop {
                    lexing_string = lexing_string.take_while(|c| c != '\\' && c != open_quote)?;

                    let (ch, escape_or_end) = lexing_string.get()?;
                    match ch {
                        '\\' => {
                            let (_, past_escape) = escape_or_end.get()?;
                            lexing_string = past_escape;
                        }
                        close if close == open_quote => {
                            break Some(self.done(TokenKind::LitString, escape_or_end));
                        }
                        _ => unreachable!()
                    }
                };

                return Some(string_loop().ok_or(LexError::UnclosedString(open_quote)));
            }

            _ if is_punct(ch) => {
                //let lexing = lexing.take_while(is_punct)?;
                self.done(TokenKind::Punct, lexing)
            }

            _ => {
                return Some(Err(LexError::BadCharacter(ch)))
            }
        };

        Some(Ok(token))
    }
}

fn is_ident_initial(ch: char) -> bool {
     is_alpha(ch) || ch == '_'
}

fn is_alpha(ch: char) -> bool {
    ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch)
}

fn is_ident_char(ch: char) -> bool {
    is_ident_initial(ch) || is_digit(ch)
}

fn is_digit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}

fn is_punct(ch: char) -> bool {
    "+-=()".contains(ch)
}

fn is_quote(ch: char) -> bool {
    ch == '\"' || ch == '\''
}

