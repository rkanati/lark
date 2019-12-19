
pub mod combinators;

#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    ParseError
}

pub type ParseResult<I, R> = Result<(R, I), ParseError>;

pub trait Parser : Sized {
    type Output;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output>;

    fn map<Func, Mapped> (self, func: Func) -> combinators::Map<Self, Func>
        where Func: Fn(Self::Output) -> Mapped
    {
        combinators::Map::new(self, func)
    }

    fn or_else<Alt> (self, alt: Alt) -> combinators::OrElse<Self, Alt>
        where Alt: Parser<Output = Self::Output>
    {
        combinators::OrElse::new(self, alt)
    }

    fn within<Open, Close> (self, open: Open, close: Close)
        -> combinators::Within<Self, Open, Close>
        where Open: Parser, Close: Parser
    {
        combinators::Within::new(self, open, close)
    }

    fn list<Separator> (self, separator: Separator) -> combinators::List<Self, Separator>
        where Separator: Parser
    {
        combinators::List::new(self, separator)
    }
}

pub trait Parse : Sized {
    fn parse<Iter> (tokens: Iter) -> ParseResult<Iter, Self>;
    fn parser() -> TrivialParser<Self> {
        TrivialParser(Default::default())
    }
}

pub struct TrivialParser<Output>(std::marker::PhantomData<Output>);

impl<O> Parser for TrivialParser<O> where O: Parse {
    type Output = O;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output> {
        O::parse(tokens)
    }
}

impl<T> Parse for Vec<T> where T: Parse {
    fn parse<Iter> (mut tokens: Iter) -> ParseResult<Iter, Self> {
        let mut vec = Vec::new();
        loop {
            match T::parse(tokens) {
                Ok((element, rest)) => {
                    vec.push(element);
                    tokens = rest;
                }
                _ => break
            }
        }
        Ok((vec, tokens))
    }
}

