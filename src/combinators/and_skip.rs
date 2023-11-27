use crate::Parse;

#[derive(Copy, Clone)]
pub struct AndSkip<P, Q> {
    parser: P,
    next: Q,
}

impl<P, Q> AndSkip<P, Q> {
    pub fn new(parser: P, next: Q) -> Self {
        Self { parser, next }
    }
}

impl<P, Q> Parse for AndSkip<P, Q>
where
    P: Parse,
    Q: Parse,
{
    type Item = P::Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser
            .parse(input)
            .and_then(|(parsed, rest)| self.next.parse(rest).map(|(_, rest)| (parsed, rest)))
    }
}
