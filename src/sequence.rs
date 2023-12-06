use crate::Parse;

#[derive(Copy, Clone)]
pub struct And<P, Q> {
    parser: P,
    next: Q,
}

impl<P, Q> And<P, Q> {
    pub fn new(parser: P, next: Q) -> Self {
        Self { parser, next }
    }
}

impl<P, Q, U> Parse for And<P, Q>
where
    P: Parse,
    Q: Parse<Item = U>,
{
    type Item = (P::Item, U);

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).and_then(|(first, rest)| {
            self.next
                .parse(rest)
                .map(|(second, rest)| ((first, second), rest))
        })
    }
}

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
