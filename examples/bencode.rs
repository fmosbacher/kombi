// use std::collections::HashMap;

// use kombi::*;

// #[derive(Debug, Clone)]
// enum Bencode {
//     Integer(i32),
//     String(String),
//     List(Vec<Bencode>),
//     Dictionary(HashMap<String, Bencode>),
// }

// fn b_integer() -> impl Parse<Item = Bencode> {
//     ch('i')
//         .and(integer())
//         .and(ch('e'))
//         .map(|((_, int), _)| Bencode::Integer(int))
// }

// fn b_string() -> impl Parse<Item = Bencode> {
//     integer()
//         .and(ch(':'))
//         .bind(|(n, _)| item().take(n as usize))
//         .map(|chars| Bencode::String(chars.iter().collect::<String>()))
// }

// fn b_list() -> impl Parse<Item = Bencode> {
//     ch('l')
//         .and(bencode().many())
//         .and(ch('e'))
//         .map(|((_, list), _)| Bencode::List(list))
// }

// fn b_dict() -> impl Parse<Item = Bencode> {
//     ch('d')
//         .and(b_string().and(bencode()).many())
//         .and(ch('e'))
//         .map(|((_, codes), _)| {
//             let mut hashmap = HashMap::new();

//             codes.into_iter().for_each(|(key, code)| {
//                 let key = match &key {
//                     Bencode::String(k) => k.clone(),
//                     _ => unreachable!(),
//                 };
//                 hashmap.insert(key, code);
//             });

//             Bencode::Dictionary(hashmap)
//         })
// }

// fn bencode() -> impl Parse<Item = Bencode> {
//     let lazy_b_list = lazy(|| Box::new(b_list()));
//     let lazy_b_dict = lazy(|| Box::new(b_dict()));
//     b_integer().or(b_string()).or(lazy_b_list).or(lazy_b_dict)
// }

// impl TryFrom<&str> for Bencode {
//     type Error = ();

//     fn try_from(value: &str) -> Result<Bencode, ()> {
//         bencode()
//             .parse(value)
//             .and_then(|(parsed, rest)| if rest.len() == 0 { Some(parsed) } else { None })
//             .ok_or(())
//     }
// }

// fn main() {
//     let text = "d8:glossaryd8:GlossDivd9:GlossListd10:GlossEntryd6:Abbrev13:ISO 8879:19867:Acronym4:SGML8:GlossDefd12:GlossSeeAlsol3:GML3:XMLe4:para72:A meta-markup language, used to create markup languages such as DocBook.e8:GlossSee6:markup9:GlossTerm36:Standard Generalized Markup Language2:ID4:SGML6:SortAs4:SGMLee5:title1:Se5:title16:example glossaryee";
//     println!("{:?}", Bencode::try_from(text));
// }

use kombi::*;

#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn hex_parser() -> impl Parse<Item = Color> {
    ch('#')
        .and(
            alphanum()
                .take(2)
                .and_then(|component| {
                    u8::from_str_radix(&component.iter().collect::<String>(), 16).ok()
                })
                .take(3),
        )
        .map(|(_, components)| Color {
            red: components[0],
            green: components[1],
            blue: components[2],
        })
}

fn main() {
    let (result, remaining) = hex_parser().parse("#6366F1").unwrap();
    println!("Result: {:?}", result);
    println!("Remaining input: {:?}", remaining);
}
