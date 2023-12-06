use crate::{
    character::{char, digit},
    Parse,
};

pub fn integer() -> impl Parse<Item = i32> + Copy {
    let neg = char('-').skip_and(digit().many1()).and_then(|digits| {
        format!("-{}", digits.iter().collect::<String>())
            .parse()
            .ok()
    });

    let pos = digit()
        .many1()
        .and_then(|digits| digits.iter().collect::<String>().parse().ok());

    neg.or(pos)
}
