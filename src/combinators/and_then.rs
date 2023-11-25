use crate::Parse;

pub struct AndThen<P, F> {
    parser: P,
    and_then_fn: F,
}

impl<P, F> AndThen<P, F> {
    pub fn new(parser: P, and_then_fn: F) -> Self {
        Self {
            parser,
            and_then_fn,
        }
    }
}

impl<P, F, U> Parse for AndThen<P, F>
where
    P: Parse,
    F: Fn(P::Item) -> Option<U>,
{
    type Item = U;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser
            .parse(input)
            .and_then(|(parsed, rest)| (self.and_then_fn)(parsed).map(|mapped| (mapped, rest)))
    }
}
