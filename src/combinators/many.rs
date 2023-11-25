use crate::Parse;

pub struct Many<P> {
    parser: P,
}

impl<P> Many<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<P> Parse for Many<P>
where
    P: Parse,
{
    type Item = Vec<P::Item>;

    fn parse<'a>(&self, mut input: &'a str) -> Option<(Self::Item, &'a str)> {
        let mut results = vec![];

        while let Some((parsed, rest)) = self.parser.parse(input) {
            results.push(parsed);
            input = rest;
        }

        Some((results, input))
    }
}
