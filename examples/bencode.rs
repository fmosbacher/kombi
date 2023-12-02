use std::collections::HashMap;

use kombi::*;

#[derive(Debug, Clone)]
enum Bencode {
    Integer(i32),
    String(String),
    List(Vec<Bencode>),
    Dictionary(HashMap<String, Bencode>),
}

fn b_integer() -> impl Parse<Item = Bencode> {
    ch('i')
        .skip_and(integer())
        .and_skip(ch('e'))
        .map(Bencode::Integer)
}

fn b_string() -> impl Parse<Item = Bencode> {
    integer()
        .and_skip(ch(':'))
        .bind(|n| {
            item()
                .take(n as usize)
                .map(|chars| chars.iter().collect::<String>())
        })
        .map(Bencode::String)
}

fn b_list() -> impl Parse<Item = Bencode> {
    ch('l')
        .and(bencode().many())
        .and(ch('e'))
        .map(|((_, list), _)| Bencode::List(list))
}

fn b_dict() -> impl Parse<Item = Bencode> {
    ch('d')
        .skip_and(b_string().and(bencode()).many())
        .and_skip(ch('e'))
        .map(|kv_pairs| {
            let mut hashmap = HashMap::new();

            kv_pairs.into_iter().for_each(|(key, value)| {
                let key = match &key {
                    Bencode::String(k) => k.clone(),
                    _ => unreachable!(),
                };

                hashmap.insert(key, value);
            });

            Bencode::Dictionary(hashmap)
        })
}

// Can't return opaque type here since it's a recursive parser
fn bencode() -> Parser<impl Fn(&str) -> Option<(Bencode, &str)>> {
    Parser::new(|input| {
        b_integer()
            .or(b_string())
            .or(b_list())
            .or(b_dict())
            .parse(input)
    })
}

impl TryFrom<&str> for Bencode {
    type Error = ();

    fn try_from(value: &str) -> Result<Bencode, ()> {
        bencode()
            .parse(value)
            .and_then(|(parsed, rest)| if rest.len() == 0 { Some(parsed) } else { None })
            .ok_or(())
    }
}

fn main() {
    let text = "d8:glossaryd8:GlossDivd9:GlossListd10:GlossEntryd6:Abbrev13:ISO 8879:19867:Acronym4:SGML8:GlossDefd12:GlossSeeAlsol3:GML3:XMLe4:para72:A meta-markup language, used to create markup languages such as DocBook.e8:GlossSee6:markup9:GlossTerm36:Standard Generalized Markup Language2:ID4:SGML6:SortAs4:SGMLee5:title1:Se5:title16:example glossaryee";
    println!("{:?}", Bencode::try_from(text));
}
