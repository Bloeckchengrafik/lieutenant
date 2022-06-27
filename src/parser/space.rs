use super::IterParser;
use anyhow::anyhow;

#[derive(Debug)]
pub enum OnceState {
    More,
    Done,
}
impl Default for OnceState {
    fn default() -> Self {
        OnceState::More
    }
}

pub struct OneOrMoreSpace;
pub struct MaybeSpaces {}

impl MaybeSpaces {
    pub fn new() -> Self {
        Self {}
    }
}

impl OneOrMoreSpace {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for OneOrMoreSpace {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MaybeSpaces {
    fn default() -> Self {
        Self::new()
    }
}

impl IterParser for OneOrMoreSpace {
    type Extract = ();

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
        let before_len = input.len();
        let out = input.trim_start();
        if out.len() == before_len {
            (Err(anyhow!("Expected a space at input '{}'", input)), None)
        } else {
            (Ok(((), out)), None)
        }
    }

    fn regex(&self) -> String {
        "\\s+".to_string()
    }
}

impl IterParser for MaybeSpaces {
    type Extract = ();
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
        let out = input.trim_start();

        (Ok(((), out)), None)
    }

    fn regex(&self) -> String {
        "\\s*".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::Command;
    use crate::parser::{IterParser, MaybeSpaces};

    #[test]
    fn test_end_of_string() {
        let input = "";
        let (out, _) = MaybeSpaces::default().parse((), input);

        assert!(out.is_ok());

        let input = " e";
        let (out, _) = MaybeSpaces::default().parse((), input);

        assert!(out.is_ok());
        assert!(
            !out.as_ref().expect("Expecting 'e'").1.is_empty(),
            "Remainder shouldn't be None for '{}', but is '{}'",
            input,
            out.unwrap().1
        );

        let command = literal("/test").space().literal("a");
        let on_call = command.on_call(|| {
            move |_| {
                println!("Called");
                0
            }
        });

        let suc = on_call.call((1,), "/test a");
        assert!(suc.is_ok(), "Expected success, but got '{:?}'", suc);

        let err = on_call.call((1,), "/test a b");
        assert!(err.is_err(), "Expected error, but got '{:?}'", err);
    }
}
