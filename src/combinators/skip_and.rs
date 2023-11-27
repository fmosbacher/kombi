use crate::Parse;

#[derive(Copy, Clone)]
pub struct SkipAnd<P, Q> {
    parser: P,
    next: Q,
}

impl<P, Q> SkipAnd<P, Q> {
    pub fn new(parser: P, next: Q) -> Self {
        Self { parser, next }
    }
}

impl<P, Q> Parse for SkipAnd<P, Q>
where
    P: Parse,
    Q: Parse,
{
    type Item = Q::Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser
            .parse(input)
            .and_then(|(_, rest)| self.next.parse(rest).map(|(parsed, rest)| (parsed, rest)))
    }
}
