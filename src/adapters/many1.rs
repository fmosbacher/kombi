use crate::Parse;

pub struct Many1<P> {
    parser: P,
}

impl<P> Many1<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }
}

impl<P> Parse for Many1<P>
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

        if results.len() > 0 {
            Some((results, input))
        } else {
            None
        }
    }
}
