use crate::Parse;

pub struct Or<P, Q> {
    parser: P,
    next: Q,
}

impl<P, Q> Or<P, Q> {
    pub fn new(parser: P, next: Q) -> Self {
        Self { parser, next }
    }
}

impl<P, Q> Parse for Or<P, Q>
where
    P: Parse,
    Q: Parse<Item = P::Item>,
{
    type Item = P::Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).or_else(|| self.next.parse(input))
    }
}
