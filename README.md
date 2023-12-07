# Kombi - Parser Combinators

This is a simple parser combinator library for Rust, providing a trait `Parse` and many combinators to build and compose parsers. Inspired by the `Iterator` trait and `Option` and `Result` methods.

### Usage

```rust
use kombi::{
    character::{alphanum, char},
    Parse,
};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn hex_parser() -> impl Parse<Item = Color> {
    // Start searching for the `#` and then ignore it
    char('#')
        .skip_and(
            // Search for 2 consecutive alphanum chars 3 times
            alphanum().take(2).take(3),
        )
        .map(|channels| {
            let mut channels = channels.iter().map(|channel| {
                u8::from_str_radix(&channel.iter().collect::<String>(), 16).unwrap()
            });
            Color {
                red: channels.next().unwrap(),
                green: channels.next().unwrap(),
                blue: channels.next().unwrap(),
            }
        })
}

fn main() {
    let (result, remaining) = hex_parser().parse("#6366F1").unwrap();
    println!("Result: {:?}", result); // Result: Color { red: 99, green: 102, blue: 241 }
    println!("Remaining input: {:?}", remaining); // Remaining input: ""
}
```

## Combinators

- **`map`:** Transforms the result of a parser using a provided function.
- **`and_then`:** Transforms the result of a parser using a provided function that returns an `Option`.
- **`and`:** Combines two parsers, applying them sequentially.
- **`or`:** Tries the first parser, and if it fails, tries the second one.
- **`many`** and **`many1`:** Apply a parser zero or more times or one or more times, respectively.
- **`bind`:** Applies a parser and then uses this result to create the next parser to apply.
- **`take`:** Takes a specified number of items from the input using the current parser.
- **`skip_and`** and **`and_skip`**: Parse a sequence ignoring the first or the second parser, respectively.

## Builtin parsers

- **`any()`:** Parses a single character.
- **`satisfy(predicate)`:** Parses a character satisfying a given predicate.
- **`char(expected)`:** Parses a specific character.
- **`digit()`:** Parses a digit character.
- **`not(unexpected)`:** Parses any character except the specified one.
- **`literal(expected)`:** Parses a literal string.
- **`whitespace()`:** Parses whitespace characters (space, newline, tab).
- **`lower()`:** Parses a lowercase letter.
- **`upper()`:** Parses an uppercase letter.
- **`alphanum()`:** Parses an alphanumeric character.
- **`integer()`:** Parses an i32 integer.
