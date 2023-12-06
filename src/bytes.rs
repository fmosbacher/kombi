use crate::{Parse, Parser};

pub fn literal(expected: &str) -> impl Parse<Item = &str> + Copy {
    Parser::new(move |input| {
        if input.starts_with(expected) {
            Some((expected, &input[expected.len()..]))
        } else {
            None
        }
    })
}
