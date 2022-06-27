use super::Argument;
use crate::parser::IterParser;

#[derive(Default)]
pub struct StringParser {}

impl IterParser for StringParser {
    type Extract = (String,);

    type ParserState = ();

    #[allow(clippy::type_complexity)]
    fn parse<'p>(
        &self,
        _state: Self::ParserState,
        input: &'p str,
    ) -> (
        anyhow::Result<(Self::Extract, &'p str)>,
        Option<Self::ParserState>,
    ) {
        let string = input.trim();

        if string.is_empty() {
            return (Err(anyhow::anyhow!("Empty input")), None);
        }

        let pos = string.find(' ').unwrap_or(string.len());

        (
            Ok(((string[0..pos].to_string(),), &string[pos..string.len()])),
            None,
        )
    }

    fn regex(&self) -> String {
        "\\S+".into()
    }
}

impl Argument for String {
    type Parser = StringParser;
    type ParserState = ();
}

#[cfg(test)]
mod tests {
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::Command;

    #[test]
    fn one_argument() {
        let command = literal("/test").space().arg::<String>();
        let x = command.on_call(|x| {
            move |expected, should_match| {
                println!("{:?}", x);
                if should_match {
                    assert_eq!(x, expected);
                }
                42
            }
        });

        let suc = x.call(("100", true), "/test 100 ");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = x.call(("ewqbe", true), "/test ewqbe");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = x.call(("test", false), "/test te st");
        assert!(err.is_err());
    }

    #[test]
    fn more_arguments() {
        let command = literal("/test")
            .space()
            .arg::<String>()
            .space()
            .arg::<String>();
        let x = command.on_call(|x, y| {
            move |expected1, expected2| {
                assert_eq!(x, expected1);
                assert_eq!(y, expected2);
                42
            }
        });

        let suc = x.call(("100", "e"), "/test 100 e");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = x.call(("ewqbe", "lalalallal312ä"), "/test ewqbe lalalallal312ä");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = x.call(("test", ""), "/test test ");
        assert!(err.is_err());
    }
}
