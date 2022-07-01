use super::Argument;
use crate::parser::IterParser;

#[derive(Default)]
pub struct U32Parser {}

#[derive(Default)]
pub struct F32Parser;

impl IterParser for U32Parser {
    type Extract = (u32,);

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
        // Consume digit from head of input

        let mut iter = input.char_indices();
        let mut index = 0;
        if let Some((i, c)) = iter.next() {
            if c == '+' || c == '-' {
                index = i;
            }
        } else {
            return (Err(anyhow::anyhow!("Empty input")), None);
        }

        for (i, c) in iter {
            if !c.is_ascii_digit() {
                break;
            }
            index = i
        }

        match input[0..=index].parse::<u32>() {
            Ok(number) => (Ok(((number,), &input[index + 1..input.len()])), None),
            Err(_) => (Err(anyhow::anyhow!("Not a number")), None),
        }
    }

    fn regex(&self) -> String {
        "[\\+|-]?\\d+".into()
    }
}

impl IterParser for F32Parser {
    type Extract = (f32,);
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

        return match string[..pos].parse::<f32>() {
            Ok(f) => (Ok(((f,), &string[pos..])), None),
            Err(e) => (
                Err(anyhow::anyhow!("Invalid input (failed to parse: {e})")),
                None,
            ),
        };
    }

    fn regex(&self) -> String {
        r"[+-]?([0-9]*[.])?[0-9]+".into() // See https://stackoverflow.com/a/12643073.
                                          // As far as i understand, this solution is not vulnerable against ReDoS
    }
}

impl Argument for u32 {
    type Parser = U32Parser;
    type ParserState = ();
}

impl Argument for f32 {
    type Parser = F32Parser;
    type ParserState = ();
}

#[cfg(test)]
mod tests {
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::Command;

    #[test]
    fn test_f32() {
        let command = literal("/test").space().arg::<f32>().on_call(|f| {
            move |_s: f32, _b: bool| {
                println!("Matching f32 {f}");
            }
        });

        let suc = command.call((123f32, true), "/test 123");
        assert!(suc.is_ok(), "{}", suc.expect_err("Expected error?"));
        let suc = command.call((0.123f32, true), "/test .123");
        assert!(suc.is_ok(), "{}", suc.expect_err("Expected error?"));
        let suc = command.call((0.123f32, true), "/test 0.123");
        assert!(suc.is_ok(), "{}", suc.expect_err("Expected error?"));
        let fail = command.call((0.123f32, false), "/test a.123");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test .");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test 123.");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test 123.123.");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test 123.123.123");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test abc");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test .a");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test a.b");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test a..b");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test ..b");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test a..");
        assert!(fail.is_err());
        let fail = command.call((0.123f32, false), "/test ..");
        assert!(fail.is_err());
    }
}
