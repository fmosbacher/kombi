use crate::{Parse, Parser};

pub fn any() -> impl Parse<Item = char> + Copy {
    Parser::new(|input| input.chars().next().map(move |first| (first, &input[1..])))
}

pub fn satisfy<F>(predicate: F) -> impl Parse<Item = char> + Copy
where
    F: Fn(char) -> bool + Copy,
{
    any().and_then(move |first| if predicate(first) { Some(first) } else { None })
}

pub fn char(expected: char) -> impl Parse<Item = char> + Copy {
    satisfy(move |first| first == expected)
}

pub fn digit() -> impl Parse<Item = char> + Copy {
    satisfy(|first| ('0'..='9').contains(&first))
}

pub fn not(expected: char) -> impl Parse<Item = char> + Copy {
    satisfy(move |first| first != expected)
}

pub fn whitespace() -> impl Parse<Item = char> + Copy + Copy {
    satisfy(|first| first.is_whitespace())
}

pub fn lower() -> impl Parse<Item = char> + Copy {
    satisfy(|first| first.is_lowercase())
}

pub fn upper() -> impl Parse<Item = char> + Copy {
    satisfy(|first| first.is_uppercase())
}

pub fn alphanum() -> impl Parse<Item = char> + Copy {
    satisfy(|first| first.is_digit(10) || first.is_lowercase() || first.is_uppercase())
}
