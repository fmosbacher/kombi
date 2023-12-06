use std::{collections::HashMap, fmt::Display};

use kombi::*;

// template ::= <replace> | <if> | <range> | <define> | <use> | <text>

type Key = String;

#[derive(Debug)]
enum Action {
    None(Key),
    Replace(Key),
    Conditional(Key, Vec<Action>),
}

impl Action {
    fn render<V>(&self, cx: &HashMap<&str, V>) -> Result<String, ()>
    where
        V: Display,
    {
        match self {
            Action::None(text) => Ok(text.to_string()),
            Action::Replace(key) => cx
                .get(key.as_str())
                .ok_or(())
                .map(|value| value.to_string()),
            Action::Conditional(key, actions) => cx
                .get(key.as_str())
                .and_then(|value| value.to_string().parse::<bool>().ok())
                .and_then(|condition| {
                    if condition {
                        actions
                            .iter()
                            .map(|action| action.render(cx))
                            .collect::<Result<String, _>>()
                            .ok()
                    } else {
                        Some(String::from(""))
                    }
                })
                .ok_or(()),
        }
    }
}

fn none_action() -> impl Parse<Item = Action> {
    not('{')
        .many1()
        .map(|chars| chars.iter().collect::<String>())
        .map(Action::None)
}

fn variable_name() -> impl Parse<Item = String> {
    lower()
        .or(ch('_'))
        .and(lower().or(alphanum()).or(ch('_')).many())
        .map(|(first, rest)| format!("{first}{rest}", rest = rest.iter().collect::<String>()))
}

fn replace_action() -> impl Parse<Item = Action> {
    ch('{')
        .skip_and(variable_name())
        .and_skip(ch('}'))
        .map(Action::Replace)
}

fn condition_action() -> impl Parse<Item = Action> {
    literal("{:if ")
        .skip_and(variable_name())
        .and_skip(ch('}'))
        .and(template())
        .and_skip(literal("{:end}"))
        .map(|(key, actions)| Action::Conditional(key, actions))
}

fn template() -> Parser<impl Fn(&str) -> Option<(Vec<Action>, &str)>> {
    Parser::new(|input| {
        replace_action()
            .or(condition_action())
            .or(none_action())
            .many()
            .parse(input)
    })
}

fn main() {
    let cx = HashMap::from([
        ("var", "123"),
        ("show", "true"),
        ("foo", "bar"),
        ("inner_show", "false"),
    ]);
    let raw = "
        {:if show}
            <p>{foo} is visible?</p>
            {:if inner_show}
                <p>{foo} again!</p>
            {:end}
        {:end}
    ";

    let Some((actions, rest)) = template().parse(raw) else {
        println!("could not even parse this shit");
        return;
    };

    println!("PARSING RESULT\nactions {:?}\nrest {:?}", actions, rest);

    if rest.len() > 0 {
        return;
    }

    let res: Result<String, _> = actions.iter().map(|action| action.render(&cx)).collect();

    println!("\nRENDERING RESULT\n{res:?}");
}
