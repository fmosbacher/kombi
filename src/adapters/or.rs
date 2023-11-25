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

impl<P, Q, T> Parse for Or<P, Q>
where
    P: Parse<Item = T>,
    Q: Parse<Item = T>,
{
    type Item = T;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).or(self.next.parse(input))
    }
}