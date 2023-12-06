use crate::Parse;

#[derive(Copy, Clone)]
pub struct Or<P, Q> {
    parser: P,
    other: Q,
}

impl<P, Q> Or<P, Q> {
    pub fn new(parser: P, other: Q) -> Self {
        Self { parser, other }
    }
}

impl<P, Q> Parse for Or<P, Q>
where
    P: Parse,
    Q: Parse<Item = P::Item>,
{
    type Item = P::Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).or_else(|| self.other.parse(input))
    }
}
