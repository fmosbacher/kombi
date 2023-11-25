mod combinators;
mod parsers;

use combinators::*;
pub use parsers::*;

pub trait Parse {
    type Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)>;

    fn map<F, T>(self, map_fn: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> T,
        Self: Sized,
    {
        Map::new(self, map_fn)
    }

    fn and<P>(self, next: P) -> And<Self, P>
    where
        P: Parse,
        Self: Sized,
    {
        And::new(self, next)
    }

    fn and_then<F, T>(self, map_fn: F) -> AndThen<Self, F>
    where
        F: Fn(Self::Item) -> Option<T>,
        Self: Sized,
    {
        AndThen::new(self, map_fn)
    }

    fn or<P>(self, next: P) -> Or<Self, P>
    where
        P: Parse,
        Self: Sized,
    {
        Or::new(self, next)
    }

    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self)
    }

    fn many1(self) -> Many1<Self>
    where
        Self: Sized,
    {
        Many1::new(self)
    }

    fn bind<P, F>(self, bind_fn: F) -> Bind<Self, F>
    where
        F: Fn(Self::Item) -> P,
        P: Parse,
        Self: Sized,
    {
        Bind::new(self, bind_fn)
    }

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, n)
    }
}

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

pub struct Lazy<F> {
    parser_builder: F,
}

pub fn lazy<P, F>(parser_builder: F) -> Lazy<Box<dyn Fn() -> Box<dyn Parse<Item = P::Item>>>>
where
    F: Fn() -> P + 'static,
    P: Parse + 'static,
{
    Lazy {
        parser_builder: Box::new(move || Box::new(parser_builder())),
    }
}

impl<T> Parse for Lazy<Box<dyn Fn() -> Box<dyn Parse<Item = T>>>> {
    type Item = T;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        (self.parser_builder)().parse(input)
    }
}
