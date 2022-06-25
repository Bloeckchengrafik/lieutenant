use crate::argument::Argument;

#[derive(Debug)]
pub struct StringWildcard {
    wildcard: String,
}

impl StringWildcard {
    pub fn new(wildcard: String) -> Self {
        Self { wildcard }
    }

    pub fn get(&self) -> &str {
        &self.wildcard
    }

    pub fn get_mut(&mut self) -> &mut str {
        &mut self.wildcard
    }
}

pub struct StringWildcardParser {}

impl crate::parser::IterParser for StringWildcardParser {
    type Extract = (StringWildcard, );
    type ParserState = ();

    #[allow(clippy::type_complexity)]
    fn parse<'p>(
        &self,
        _state: Self::ParserState,
        input: &'p str,
    ) -> (
        anyhow::Result<((StringWildcard, ), &'p str)>,
        Option<Self::ParserState>,
    ) {
        let string = input.trim();

        if string.len() == 0 {
            return (Err(anyhow::anyhow!("Empty input")), None);
        }
        (Ok(((StringWildcard { wildcard: string.to_string() }, ), "")), None)

    }

    fn regex(&self) -> String {
        ".*".into()
    }
}

impl Default for StringWildcardParser {
    fn default() -> Self {
        StringWildcardParser {}
    }
}

impl Argument for StringWildcard {
    type Parser = StringWildcardParser;
    type ParserState = ();
}

#[cfg(test)]
mod tests {
    use crate::argument::{StringWildcard};
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::Command;

    #[test]
    fn one_argument() {
        let command = literal("/test").space().arg::<StringWildcard>();
        let x = command.on_call(|x| {
            move |expected, _| {
                println!("{:?}", x);
                //assert_eq!(x, expected);
                42
            }
        });

        let suc = x.call(("test", 1), "/test test");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = x.call(("täst test test", 1), "/test täst test test");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = x.call(("", 1), "/test ");
        assert!(err.is_err(), "{:?}", err);

        let err = x.call(("", 1), "/test");
        assert!(err.is_err(), "{:?}", err);
    }
}