use crate::Parse;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
