use crate::Parse;

#[derive(Copy, Clone)]
pub struct Take<P> {
    parser: P,
    n: usize,
}

impl<P> Take<P> {
    pub fn new(parser: P, n: usize) -> Self {
        Self { parser, n }
    }
}

impl<P> Parse for Take<P>
where
    P: Parse,
{
    type Item = Vec<P::Item>;

    fn parse<'a>(&self, mut input: &'a str) -> Option<(Self::Item, &'a str)> {
        if self.n <= 0 {
            return None;
        }

        let mut results = vec![];

        for _ in 0..self.n {
            if let Some((parsed, rest)) = self.parser.parse(input) {
                results.push(parsed);
                input = rest;
            } else {
                break;
            }
        }

        if results.len() != self.n {
            return None;
        }

        Some((results, input))
    }
}
