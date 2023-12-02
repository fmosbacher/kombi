# Kombi - Parser Combinators

This is a simple parser combinator library for Rust, providing a trait `Parse` and many combinators to build and compose parsers. Inspired by the `Iterator` trait and some `Option` methods.

### Usage

```rust
use kombi::*;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn hex_parser() -> impl Parse<Item = Color> {
    // Start searching for the `#`
    ch('#')
        .skip_and(
            // Search for 2 consecutive alphanum chars and parse them into a u8 number
            alphanum()
                .take(2)
                .and_then(|component| {
                    u8::from_str_radix(&component.iter().collect::<String>(), 16).ok()
                })
                // Do the previous parser (2 alphanum -> u8) 3 times
                .take(3),
        )
        // Ignore the `#` and use the list of 3 numbers to build the Color type
        .and_then(|components| {
            let mut components = components.iter();
            let (Some(&red), Some(&green), Some(&blue)) =
                (components.next(), components.next(), components.next())
            else {
                return None;
            };
            Some(Color { red, green, blue })
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
- **`many` and `many1`:** Apply a parser zero or more times or one or more times, respectively.
- **`bind`:** Applies a parser and then uses this result to create the next parser to apply.
- **`take`:** Takes a specified number of items from the input using the current parser.
- **`lazy`:** Creates a lazy parser for deferred evaluation.

## Builtin parsers

- **`item()`:** Parses a single character.
- **`sat(predicate)`:** Parses a character satisfying a given predicate.
- **`ch(expected)`:** Parses a specific character.
- **`digit()`:** Parses a digit character.
- **`not(expected)`:** Parses any character except the specified one.
- **`literal(expected)`:** Parses a literal string.
- **`whitespace()`:** Parses whitespace characters (space, newline, tab).
- **`lower()`:** Parses a lowercase letter.
- **`upper()`:** Parses an uppercase letter.
- **`alphanum()`:** Parses an alphanumeric character.
- **`integer()`:** Parses an integer.
