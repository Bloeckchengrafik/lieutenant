use super::CommandSpec;
use crate::{
    argument::Argument,
    generic::Func,
    parser::{self, And, IterParser, MaybeSpaces, OneOrMoreSpace},
};

// use std::marker::PhantomData;
pub fn space() -> OneOrMoreSpace {
    OneOrMoreSpace
}

pub fn literal(value: &str) -> parser::Literal {
    parser::Literal {
        value: String::from(value),
    }
}

pub trait CommandBuilder {
    type Parser: IterParser;
    fn arg<A: Argument>(self) -> And<Self::Parser, <A as Argument>::Parser>;
    fn space(self) -> And<Self::Parser, OneOrMoreSpace>;
    fn literal(self, literal: &str) -> And<Self::Parser, parser::Literal>;
    fn followed_by<P: IterParser>(self, parser: P) -> And<Self::Parser, P>;
    fn on_call<GameState, CommandResult, F1, F2>(
        self,
        f: F1,
    ) -> CommandSpec<GameState, CommandResult, F1, F2, And<Self::Parser, MaybeSpaces>>
    where
        F1: Func<<Self::Parser as IterParser>::Extract, Output = F2>,
        F2: Func<GameState, Output = CommandResult>;
}

impl<T> CommandBuilder for T
where
    T: IterParser,
{
    type Parser = T;

    fn arg<A: Argument>(self) -> And<Self::Parser, A::Parser> {
        And {
            a: self,
            b: A::Parser::default(),
        }
    }

    fn space(self) -> And<Self::Parser, OneOrMoreSpace> {
        self.followed_by(space())
    }

    fn literal(self, str: &str) -> And<Self::Parser, parser::Literal> {
        self.followed_by(literal(str))
    }

    fn followed_by<P: IterParser>(self, other: P) -> And<Self::Parser, P> {
        And { a: self, b: other }
    }

    fn on_call<GameState, CommandResult, F1, F2>(
        self,
        f: F1,
    ) -> CommandSpec<GameState, CommandResult, F1, F2, And<Self::Parser, MaybeSpaces>>
    where
        F1: Func<<Self::Parser as IterParser>::Extract, Output = F2>,
        F2: Func<GameState, Output = CommandResult>,
    {
        CommandSpec {
            parser: self.followed_by(MaybeSpaces{nothing_should_follow: true}),
            mapping: f,
            gamestate: Default::default(),
            command_result: Default::default(),
            mapping_result: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::{Command, CommandSpec};

    use super::{literal, CommandBuilder};

    #[test]
    fn escape_literal() {
        let lit = literal("/echo").value;
        println!("lit:{:?}", lit);
    }

    #[test]
    fn multiple_args() {
        let command = literal("/args")
            .space()
            .arg::<u32>()
            .space()
            .arg::<u32>()
            .on_call(|x, y| {
                move |s: &str, u: usize | {
                    println!("multiple args with {x} {y} {s} {u}");
                }
            });
        let suc = command.call(("Hello", 10), "/args 42 42");
        assert!(suc.is_ok(), "{:?}", suc);
        let suc = command.call(("Hello", 10), "/args 24 42");
        assert!(suc.is_ok(), "{:?}", suc);
        let fail = command.call(("Hello", 10), "/args abc 42");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/args abc abc");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/args 42 abc");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/args 42");
        assert!(fail.is_err());

    }

    #[test]
    fn multiple_literals() {
        let command = literal("/lit")
            .space()
            .literal("literal")
            .on_call(|| {
                move |s: &str, u: usize| {
                    println!("{s}, {u}");
                }
            });
        let suc = command.call(("Hello", 10), "/lit literal");
        let fail = command.call(("Hello", 10), "/lit fail");
        let much = command.call(("Hello", 10), "/lit literal another");
        assert!(suc.is_ok());
        assert!(fail.is_err());
        assert!(much.is_err()); // We agreed on not tolerating args after the end of the command.
    }

    #[test]
    fn multiple_duplicate_literals() {
        let command  = literal("/lit")
            .space()
            .literal("lit")
            .on_call(|| {
                move |s: &str, u: usize| {
                    println!("{s}, {u}");
                }
            });
        let suc = command.call(("Hello", 10), "/lit lit");
        let fail = command.call(("Hello", 10), "/lit notlit");
        let much = command.call(("Hello", 10), "/lit lit lit");
        assert!(suc.is_ok());
        assert!(fail.is_err());
        assert!(much.is_err()); // See above.
    }

    #[test]
    fn multiple_literals_with_args() {
        let command = literal("/lit")
            .space()
            .arg::<u32>()
            .space()
            .literal("literal")
            .on_call(|arg: u32| {
                move |s: &str, u: usize| {
                    println!("Multiple literals with args {arg} {s} {u}");
                }
            });

        let suc = command.call(("Hello", 10), "/lit 42 literal");
        assert!(suc.is_ok());
        let fail = command.call(("Hello", 10), "/lit literal");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit literal 42");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit 42");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit literal");
        assert!(fail.is_err());
        let fail = command.call(("Hello", 10), "/lit 42 lit");
        assert!(fail.is_err());

    }

    #[test]
    fn case() {
        let cmd: CommandSpec<(&mut usize, &mut usize), usize, _, _, _> = literal("/echo")
            .space()
            .arg::<u32>()
            .on_call(|arg: u32| move |_x: &mut usize, _y: &mut usize| arg as usize);

        let x = &mut 10;
        let y = &mut 100;
        assert!(cmd.call((x, y), "/echo 10").is_ok());
        println!("{:?}", cmd.call((x, y), "/echo 10 "));
    }
}
