use crate::{Parse, Parser};

pub fn item() -> impl Parse<Item = char> + Copy {
    Parser::new(|input| input.chars().next().map(move |first| (first, &input[1..])))
}

pub fn sat<F>(predicate: F) -> impl Parse<Item = char> + Copy
where
    F: Fn(char) -> bool + Copy,
{
    item().and_then(move |first| if predicate(first) { Some(first) } else { None })
}

pub fn ch(expected: char) -> impl Parse<Item = char> + Copy {
    sat(move |first| first == expected)
}

pub fn digit() -> impl Parse<Item = char> + Copy {
    sat(|first| ('0'..='9').contains(&first))
}

pub fn not(expected: char) -> impl Parse<Item = char> + Copy {
    sat(move |first| first != expected)
}

pub fn literal(expected: &str) -> impl Parse<Item = &str> + Copy {
    Parser::new(move |input| {
        if input.starts_with(expected) {
            Some((expected, &input[expected.len()..]))
        } else {
            None
        }
    })
}

pub fn whitespace() -> impl Parse<Item = char> + Copy + Copy {
    sat(|first| first.is_whitespace())
}

pub fn lower() -> impl Parse<Item = char> + Copy {
    sat(|first| first.is_lowercase())
}

pub fn upper() -> impl Parse<Item = char> + Copy {
    sat(|first| first.is_uppercase())
}

pub fn alphanum() -> impl Parse<Item = char> + Copy {
    sat(|first| first.is_digit(10) || first.is_lowercase() || first.is_uppercase())
}

pub fn integer() -> impl Parse<Item = i32> + Copy {
    let neg = ch('-').skip_and(digit().many1()).and_then(|digits| {
        format!("-{}", digits.iter().collect::<String>())
            .parse()
            .ok()
    });

    let pos = digit()
        .many1()
        .and_then(|digits| digits.iter().collect::<String>().parse().ok());

    neg.or(pos)
}
