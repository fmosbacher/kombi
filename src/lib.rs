mod combinators;
mod parsers;

use combinators::*;
pub use parsers::*;

pub trait Parse: Sized {
    type Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)>;

    fn map<F, T>(self, map_fn: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> T,
    {
        Map::new(self, map_fn)
    }

    fn and<P>(self, next: P) -> And<Self, P>
    where
        P: Parse,
    {
        And::new(self, next)
    }

    fn and_then<F, T>(self, map_fn: F) -> AndThen<Self, F>
    where
        F: Fn(Self::Item) -> Option<T>,
    {
        AndThen::new(self, map_fn)
    }

    fn or<P>(self, next: P) -> Or<Self, P>
    where
        P: Parse,
    {
        Or::new(self, next)
    }

    fn many(self) -> Many<Self> {
        Many::new(self)
    }

    fn many1(self) -> Many1<Self> {
        Many1::new(self)
    }

    fn bind<P, F>(self, bind_fn: F) -> Bind<Self, F>
    where
        F: Fn(Self::Item) -> P,
        P: Parse,
    {
        Bind::new(self, bind_fn)
    }

    fn take(self, n: usize) -> Take<Self> {
        Take::new(self, n)
    }

    fn and_skip<P>(self, next: P) -> AndSkip<Self, P>
    where
        P: Parse,
    {
        AndSkip::new(self, next)
    }

    fn skip_and<P>(self, next: P) -> SkipAnd<Self, P>
    where
        P: Parse,
    {
        SkipAnd::new(self, next)
    }
}

#[derive(Copy, Clone)]
pub struct Parser<F>(F);

impl<F, T> Parser<F>
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    pub fn new(f: F) -> Parser<F> {
        Parser(f)
    }
}

impl<F, T> Parse for Parser<F>
where
    F: Fn(&str) -> Option<(T, &str)>,
{
    type Item = T;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.0(input)
    }
}
