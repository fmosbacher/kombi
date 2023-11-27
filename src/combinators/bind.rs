use crate::Parse;

#[derive(Copy, Clone)]
pub struct Bind<P, F> {
    parser: P,
    bind_fn: F,
}

impl<P, F> Bind<P, F> {
    pub fn new(parser: P, bind_fn: F) -> Self {
        Self { parser, bind_fn }
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
        self.parser
            .parse(input)
            .and_then(|(parsed, rest)| (self.bind_fn)(parsed).parse(rest))
    }
}
