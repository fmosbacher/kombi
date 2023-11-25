pub trait Parse {
    type Item;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)>;

    fn map<F, T>(self, map_fn: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> T,
        Self: Sized,
    {
        Map {
            parser: self,
            map_fn,
        }
    }

    fn and<P>(self, next: P) -> And<Self, P>
    where
        P: Parse,
        Self: Sized,
    {
        And { parser: self, next }
    }

    fn and_then<F, T>(self, map_fn: F) -> AndThen<Self, F>
    where
        F: Fn(Self::Item) -> Option<T>,
        Self: Sized,
    {
        AndThen {
            parser: self,
            map_fn,
        }
    }

    fn or<P>(self, next: P) -> Or<Self, P>
    where
        P: Parse,
        Self: Sized,
    {
        Or { parser: self, next }
    }

    fn many(self) -> Many<Self>
    where
        Self: Sized,
    {
        Many { parser: self }
    }

    fn many1(self) -> Many1<Self>
    where
        Self: Sized,
    {
        Many1 { parser: self }
    }

    fn bind<P, F>(self, bind_fn: F) -> Bind<Self, F>
    where
        F: Fn(Self::Item) -> P,
        P: Parse,
        Self: Sized,
    {
        Bind {
            parser: self,
            bind_fn,
        }
    }

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take { parser: self, n }
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

pub struct Map<P, F> {
    parser: P,
    map_fn: F,
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

pub struct And<P, Q> {
    parser: P,
    next: Q,
}

impl<P, Q, U> Parse for And<P, Q>
where
    P: Parse,
    Q: Parse<Item = U>,
{
    type Item = (P::Item, U);

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).and_then(|(first, rest)| {
            self.next
                .parse(rest)
                .map(|(second, rest)| ((first, second), rest))
        })
    }
}

pub struct AndThen<P, F> {
    parser: P,
    map_fn: F,
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
            .and_then(|(parsed, rest)| (self.map_fn)(parsed).map(|mapped| (mapped, rest)))
    }
}

pub struct Or<P, Q> {
    parser: P,
    next: Q,
}

impl<P, Q, T> Parse for Or<P, Q>
where
    P: Parse<Item = T>,
    Q: Parse<Item = T>,
{
    type Item = T;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).or(self.next.parse(input))
    }
}

pub struct Many<P> {
    parser: P,
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

pub struct Many1<P> {
    parser: P,
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

pub struct Bind<P, F> {
    parser: P,
    bind_fn: F,
}

impl<P, Q, F, U> Parse for Bind<P, F>
where
    P: Parse,
    F: Fn(P::Item) -> Q,
    Q: Parse<Item = U>,
{
    type Item = U;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        self.parser.parse(input).and_then(|(parsed, rest)| {
            let q = (self.bind_fn)(parsed);
            q.parse(rest).map(|(parsed, rest)| (parsed, rest))
        })
    }
}

pub struct Take<P> {
    parser: P,
    n: usize,
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

pub struct LazyParser<T>(Box<dyn Fn() -> Box<dyn Parse<Item = T>>>);

impl<T> Parse for LazyParser<T> {
    type Item = T;

    fn parse<'a>(&self, input: &'a str) -> Option<(Self::Item, &'a str)> {
        (self.0)().parse(input)
    }
}

pub fn lazy<F, T>(f: F) -> LazyParser<T>
where
    F: Fn() -> Box<dyn Parse<Item = T>> + 'static,
{
    LazyParser(Box::new(move || f()))
}

pub fn item() -> impl Parse<Item = char> {
    Parser::new(|input: &str| input.chars().next().map(|first| (first, &input[1..])))
}

pub fn sat<F>(predicate: F) -> impl Parse<Item = char>
where
    F: Fn(char) -> bool,
{
    item().and_then(move |first| if predicate(first) { Some(first) } else { None })
}

pub fn ch(search: char) -> impl Parse<Item = char> {
    sat(move |first| first == search)
}

pub fn digit() -> impl Parse<Item = char> {
    sat(|first| ('0'..='9').contains(&first))
}

pub fn not(search: char) -> impl Parse<Item = char> {
    sat(move |first| first != search)
}

pub fn literal(search: &str) -> impl Parse<Item = &str> {
    Parser::new(move |input| {
        if input.starts_with(search) {
            Some((search, &input[search.len()..]))
        } else {
            None
        }
    })
}

pub fn whitespace() -> impl Parse<Item = char> {
    sat(|first| first == ' ' || first == '\n' || first == '\t')
}

pub fn lower() -> impl Parse<Item = char> {
    sat(|first| ('a'..='z').contains(&first))
}

pub fn upper() -> impl Parse<Item = char> {
    sat(|first| ('A'..='Z').contains(&first))
}

pub fn alphanum() -> impl Parse<Item = char> {
    sat(|first| {
        ('a'..='z').contains(&first) || ('0'..='9').contains(&first) || ('A'..='Z').contains(&first)
    })
}

pub fn integer() -> impl Parse<Item = i32> {
    let neg = ch('-').and(digit().many1()).and_then(|(_, digits)| {
        format!("-{}", digits.iter().collect::<String>())
            .parse()
            .ok()
    });

    let pos = digit()
        .many1()
        .and_then(|digits| digits.iter().collect::<String>().parse().ok());

    neg.or(pos)
}
