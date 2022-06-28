use crate::parser::IterParser;

pub struct ChoiceParser {
    choices: Vec<String>,
}

impl ChoiceParser {
    pub fn new(choices: Vec<String>) -> ChoiceParser {
        ChoiceParser { choices }
    }
}

impl IterParser for ChoiceParser {
    type Extract = (String,);
    type ParserState = ();

    fn parse<'p>(
        &self,
        _: Self::ParserState,
        input: &'p str,
    ) -> (
        anyhow::Result<(Self::Extract, &'p str)>,
        Option<Self::ParserState>,
    ) {
        for choice in &self.choices {
            if (input.len() == choice.len() && input == choice)
                || (input.starts_with(&(choice.to_owned() + " ")))
            {
                return (Ok(((String::from(choice),), &input[choice.len()..])), None);
            }
        }
        (Err(anyhow::anyhow!("No Choice matched")), None)
    }

    fn regex(&self) -> String {
        self.choices
            .iter()
            .clone()
            .map(|x| {
                x.replace('\\', "\\\\")
                    .replace('.', "\\.")
                    .replace('^', "\\^")
                    .replace('$', "\\$")
                    .replace('|', "\\|")
                    .replace('*', "\\*")
                    .replace('+', "\\+")
                    .replace('?', "\\?")
                    .replace('{', "\\{")
                    .replace('}', "\\}")
                    .replace('(', "\\(")
                    .replace(')', "\\)")
                    .replace('[', "\\[")
                    .replace(']', "\\]")
            })
            .collect::<Vec<String>>()
            .join("|")
    }
}

#[cfg(test)]
mod tests {
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::Command;

    #[test]
    fn simple() {
        let choices: Vec<String> = vec!["e".into(), "f\\r".into(), "minecraft:chicken".into()];
        let command = literal("/test")
            .space()
            .choice(choices)
            .on_call(|x: String| {
                move |_, _| {
                    println!("{}", x);
                    42
                }
            });

        let suc = command.call(("Hello", 10), "/test f\\r");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test e");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test minecraft:chicken");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = command.call(("Hello", 10), "/test g");
        assert!(err.is_err(), "{:?}", err);

        let err = command.call(("Hello", 10), "/test f\\r\\r");
        assert!(err.is_err(), "{:?}", err);

        let err = command.call(("Hello", 10), "/test f\\r\\r\\r");
        assert!(err.is_err(), "{:?}", err);
    }

    #[test]
    fn multiple() {
        let choices: Vec<String> = vec!["e".into(), "f\\r".into(), "minecraft:chicken.".into()];
        let command = literal("/test")
            .space()
            .choice(choices.clone())
            .space()
            .choice(choices)
            .on_call(|x: String, y: String| {
                move |_, _| {
                    println!("x={}", x);
                    println!("y={}", y);
                    42
                }
            });

        let suc = command.call(("Hello", 10), "/test f\\r e");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test f\\r minecraft:chicken.");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test f\\r f\\r");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = command.call(("Hello", 10), "/test f\\r f\\r minecraft:chicken");
        assert!(err.is_err(), "{:?}", err);

        let err = command.call(("Hello", 10), "/test f\\r f\\r f\\r");
        assert!(err.is_err(), "{:?}", err);
    }

    #[test]
    fn optional() {
        let choices: Vec<String> = vec!["e".into(), "f\\r".into(), "minecraft:chicken.".into()];
        let command =
            literal("/test")
                .opt_space()
                .opt_choice(choices)
                .on_call(|x: Option<(String,)>| {
                    move |_, _| {
                        dbg!(&x);
                        42
                    }
                });

        let suc = command.call(("Hello", 10), "/test f\\r");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test e");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test ");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = command.call(("Hello", 10), "/test f\\r\\r");
        assert!(err.is_err(), "{:?}", err);
    }

    #[test]
    fn multiple_optional() {
        let choices: Vec<String> = vec!["e".into(), "f\\r".into(), "minecraft:chicken.".into()];
        let command = literal("/test")
            .opt_space()
            .opt_choice(choices.clone())
            .opt_space()
            .opt_choice(choices)
            .on_call(|x: Option<(String,)>, y: Option<(String,)>| {
                move |_, _| {
                    dbg!(&x);
                    dbg!(&y);
                    42
                }
            });

        let suc = command.call(("Hello", 10), "/test f\\r e");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test f\\r minecraft:chicken.");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test f\\r");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test f\\r ");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test ");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = command.call(("Hello", 10), "/test");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = command.call(("Hello", 10), "/test f\\r fä\\r");
        assert!(err.is_err(), "{:?}", err);

        let err = command.call(("Hello", 10), "/test f\\r f\\r f\\r");
        assert!(err.is_err(), "{:?}", err);

        let err = command.call(("Hello", 10), "/test fä\\r f\\r");
        assert!(err.is_err(), "{:?}", err);
    }
}
