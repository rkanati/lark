
#[derive(Clone, Copy, Debug)]
pub struct LexMatch<'a> {
    pub leader:   &'a str,
    pub spelling: &'a str,
}

#[derive(Clone, Copy)]
pub struct Lexing<'a> {
    line:  &'a str,
    start: usize,
    end:   usize,
}

impl<'a> Lexing<'a> {
    pub fn new(line: &'a str) -> Lexing<'a> {
        Lexing { line, start: 0, end: 0 }
    }

    pub fn consume(self) -> (LexMatch<'a>, Lexing<'a>) {
        let leader   = &self.line[           .. self.start];
        let spelling = &self.line[self.start .. self.end];
        let mat = LexMatch { leader, spelling };
        let rest = Lexing { start: self.end, ..self };
        (mat, rest)
    }

    pub fn discard(self) -> Lexing<'a> {
        let (_, rest) = self.consume();
        rest
    }

    pub fn get(self) -> Option<(char, Self)> {
        self.line[self.end ..]
            .chars()
            .next()
            .map(|ch| (ch, Lexing { end: self.end + ch.len_utf8(), ..self }))
    }

    pub fn take(self, n: usize) -> Option<Self> {
        let end = self.end + n;
        if end <= self.line.len() {
            Some(Lexing { end, ..self })
        }
        else {
            None
        }
    }

    pub fn take_while(mut self, mut pred: impl FnMut(char) -> bool) -> Option<Self> {
        let mut taken = None;
        while let Some((head, tail)) = self.get().filter(|(head, _)| pred(*head)) {
            self = tail;
            taken = Some(tail);
        }
        taken
    }
}

