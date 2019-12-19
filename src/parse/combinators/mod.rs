
use super::*;

pub struct Map<Inner, Func> {
    inner: Inner,
    func:  Func
}

impl<I, F> Map<I, F> {
    pub fn new(inner: I, func: F) -> Map<I, F> {
        Map { inner, func }
    }
}

impl<I, F, R> Parser for Map<I, F>
    where I: Parser,
          F: FnOnce(<I as Parser>::Output) -> R
{
    type Output = R;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output> {
        self.inner.parse(tokens)
            .map(|(value, tokens)| ((self.func)(value), tokens))
    }
}



pub struct OrElse<Left, Right> {
    left:  Left,
    right: Right
}

impl<L, R> OrElse<L, R> {
    pub fn new(left: L, right: R) -> OrElse<L, R> {
        OrElse { left, right }
    }
}

impl<L, R, O> Parser for OrElse<L, R>
    where L: Parser<Output = O>,
            R: Parser<Output = O>
{
    type Output = O;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output> {
        self.left.parse(tokens)
            .or_else(|_| self.right.parse(tokens))
    }
}





pub struct Within<Inner, Open, Close> {
    inner: Inner,
    open:  Open,
    close: Close,
}

impl<I, O, C> Within<I, O, C> {
    pub fn new(inner: I, open: O, close: C) -> Within<I, O, C> {
        Within { inner, open, close }
    }
}

impl<I, O, C> Parser for Within<I, O, C>
    where I: Parser, O: Parser, C: Parser
{
    type Output = <I as Parser>::Output;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output> {
        let (_,      tokens) = self.open. parse(tokens)?;
        let (result, tokens) = self.inner.parse(tokens)?;
        let (_,      tokens) = self.close.parse(tokens)?;
        Ok((result, tokens))
    }
}




pub struct List<Element, Separator> {
    element:   Element,
    separator: Separator,
}

impl<E, S> List<E, S> {
    pub fn new(element: E, separator: S) -> List<E, S> {
        List { element, separator }
    }
}

impl<E, S> Parser for List<E, S>
    where E: Parser, S: Parser
{
    type Output = Vec<<E as Parser>::Output>;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output> {
        let mut elements = Vec::new();

        loop {
            let (element, rest) = match self.element.parse(tokens) {
                Ok(tup) => tup,
                _       => break
            };
            elements.push(element);

            tokens = match self.separator.parse(rest) {
                Ok((_, tokens)) => tokens,
                _               => break
            };
        }

        Ok((elements, tokens))
    }
}





pub struct FirstOf<Output> {
    alternatives: &'static [&'static dyn Parser<Output = Output>],
}

impl<O> FirstOf<O> {
    pub fn new(alternatives: &'static [&'static dyn Parser<Output = O>]) -> FirstOf<O> {
        FirstOf { alternatives }
    }
}

impl<O> Parser for FirstOf<O> {
    type Output = O;
    fn parse<Iter> (&self, tokens: Iter) -> ParseResult<Iter, Self::Output> {
        for alternative in self.alternatives.iter() {
            if let Ok(tup) = alternative.parse(tokens) {
                return tup;
            }
        }

        Err(ParseError::ParseError)
    }
}

