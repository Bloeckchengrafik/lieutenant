use crate::parser::IterParser;
use super::Argument;

pub struct StringParser {}

impl IterParser for StringParser {
    type Extract = (String, );

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

        if string.len() == 0 {
            return (Err(anyhow::anyhow!("Empty input")), None);
        }

        let pos = string.find(" ").unwrap_or(string.len());

        (Ok(((string[0..pos].to_string(), ), &string[pos..string.len()])), None)
    }

    fn regex(&self) -> String {
        "\\S+".into()
    }
}

impl Default for StringParser {
    fn default() -> Self {
        Self {}
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
            move |expected, _| {
                println!("{:?}", x);
                assert_eq!(x, expected);
                42
            }
        });

        let suc = x.call(("100", 1), "/test 100 ");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = x.call(("ewqbe", 1), "/test ewqbe");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = x.call(("test", 1), "/test te st");
        assert!(err.is_err());
    }

    #[test]
    fn more_arguments() {
        let command = literal("/test").space().arg::<String>().space().arg::<String>();
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
