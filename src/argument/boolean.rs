use super::Argument;
use crate::parser::IterParser;

#[derive(Default)]
pub struct BoolParser;

impl IterParser for BoolParser {
    type Extract = (bool, );
    type ParserState = ();

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

        return match &string[..pos] {
            "false" => (Ok(((false, ), &string[pos..string.len()])), None),
            "true" => (Ok(((true, ), &string[pos..string.len()])), None),
            _ => (Err(anyhow::anyhow!("Invalid input (not a boolean)")), None),
        };
    }

    fn regex(&self) -> String {
        "(true|false)".into()
    }
}

impl Argument for bool {
    type Parser = BoolParser;
    type ParserState = ();
}

#[cfg(test)]
mod tests {
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::{Command, CommandSpec};

    #[test]
    fn test_boolean() {
        let command = literal("/lit").space().arg::<bool>().on_call(|arg: bool| {
            move |_s: &str, _u: usize| {
                println!("boolean argument {arg}");
            }
        });

        let suc = command.call(("Hello", 10), "/lit false");
        assert!(suc.is_ok());
        let suc = command.call(("Hello", 10), "/lit true");
        assert!(suc.is_ok());
        let fail = command.call(("Hello", 10), "/lit 1234");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit dings");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit fals");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit tru");
        assert!(fail.is_err());
    }

    #[allow(clippy::bool_comparison)]
    #[test]
    fn bool_with_other_args() {
        let command: CommandSpec<(&str, bool), bool, _, _, _> = literal("/test")
            .space()
            .arg::<u32>()
            .space()
            .arg::<bool>()
            .space()
            .arg::<String>()
            .on_call(|u, b, s| {
                move |expected: &str, should_match: bool| {
                    println!("Multiple arguments {u},{b},{s} = {expected}/{should_match}");
                    b
                }
            });

        let suc = command.call(("123 false test", true), "/test 123 false test");
        assert!(suc.is_ok() && suc.unwrap() == false); // not replaced with a negation to improve readability
        let suc = command.call(("1234 true test", true), "/test 1234 true test");
        assert!(suc.is_ok() && suc.unwrap() == true); // adding comparison to true to improve readability
        let suc = command.call(("1234 true test", true), "/test 123 true test");
        assert!(suc.is_ok() && suc.unwrap() == true);
        let suc = command.call(("1234 false test", true), "/test 1234 false test");
        assert!(suc.is_ok() && suc.unwrap() == false);

        let fail = command.call(("1234 test true", false), "/test 1234 test true");
        assert!(fail.is_err());
        let fail = command.call(("test 1234 true", false), "/test test 1234 true");
        assert!(fail.is_err());
        let fail = command.call(("true 1234 test", false), "/test true 1234 test");
        assert!(fail.is_err());
        let fail = command.call(("true test 1234", false), "/test true test 1234");
        assert!(fail.is_err());
        let fail = command.call(("1234 test false", false), "/test 1234 test false");
        assert!(fail.is_err());
        let fail = command.call(("test 1234 false", false), "/test test 1234 false");
        assert!(fail.is_err());
        let fail = command.call(("false 1234 test", false), "/test false 1234 test");
        assert!(fail.is_err());
        let fail = command.call(("false test 1234", false), "/test false test 1234");
        assert!(fail.is_err());
    }


    #[test]
    fn bool_split() {
        let command = literal("/test").space().arg::<bool>()
            .on_call(|arg: bool| {
                move |_s: &str, _u: bool| {
                    println!("boolean argument {arg}");
                }
            });

        let suc = command.call(("fa lse", false), "/lit fa lse");
        assert!(suc.is_err());
        let suc = command.call(("tr ue", false), "/lit tr ue");
        assert!(suc.is_err());
    }
}
