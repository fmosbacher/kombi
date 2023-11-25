use crate::{Parse, Parser};

pub fn item() -> impl Parse<Item = char> {
    Parser::new(|input: &str| input.chars().next().map(|first| (first, &input[1..])))
}

pub fn sat<F>(predicate: F) -> impl Parse<Item = char>
where
    F: Fn(char) -> bool,
{
    item().and_then(move |first| if predicate(first) { Some(first) } else { None })
}

pub fn ch(expected: char) -> impl Parse<Item = char> {
    sat(move |first| first == expected)
}

pub fn digit() -> impl Parse<Item = char> {
    sat(|first| ('0'..='9').contains(&first))
}

pub fn not(expected: char) -> impl Parse<Item = char> {
    sat(move |first| first != expected)
}

pub fn literal(expected: &str) -> impl Parse<Item = &str> {
    Parser::new(move |input| {
        if input.starts_with(expected) {
            Some((expected, &input[expected.len()..]))
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
