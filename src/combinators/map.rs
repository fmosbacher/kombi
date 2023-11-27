use crate::Parse;

#[derive(Copy, Clone)]
pub struct Map<P, F> {
    parser: P,
    map_fn: F,
}

impl<P, F> Map<P, F> {
    pub fn new(parser: P, map_fn: F) -> Self {
        Self { parser, map_fn }
    }
}

impl<P, F, U> Parse for Map<P, F>
where
    P: Parse,
    F: Fn(P::Item) -> U,
{
    type Item = U;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser
            .parse(input)
            .map(|(parsed, rest)| ((self.map_fn)(parsed), rest))
    }
}
