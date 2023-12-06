use crate::Parse;

#[derive(Copy, Clone)]
pub struct Map<P, F>(P, F);

impl<P, F> Map<P, F> {
    pub fn new(parser: P, map_fn: F) -> Self {
        Self(parser, map_fn)
    }
}

impl<P, F, U> Parse for Map<P, F>
where
    P: Parse,
    F: Fn(P::Item) -> U,
{
    type Item = U;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.0
            .parse(input)
            .map(|(parsed, rest)| ((self.1)(parsed), rest))
    }
}

#[derive(Copy, Clone)]
pub struct AndThen<P, F>(P, F);

impl<P, F> AndThen<P, F> {
    pub fn new(parser: P, and_then_fn: F) -> Self {
        Self(parser, and_then_fn)
    }
}

impl<P, F, U> Parse for AndThen<P, F>
where
    P: Parse,
    F: Fn(P::Item) -> Option<U>,
{
    type Item = U;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.0
            .parse(input)
            .and_then(|(parsed, rest)| (self.1)(parsed).map(|mapped| (mapped, rest)))
    }
}

#[derive(Copy, Clone)]
pub struct Bind<P, F>(P, F);

impl<P, F> Bind<P, F> {
    pub fn new(parser: P, bind_fn: F) -> Self {
        Self(parser, bind_fn)
    }
}

impl<P, Q, F, U> Parse for Bind<P, F>
where
    P: Parse,
    F: Fn(P::Item) -> Q,
    Q: Parse<Item = U>,
{
    type Item = U;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.0
            .parse(input)
            .and_then(|(parsed, rest)| (self.1)(parsed).parse(rest))
    }
}
