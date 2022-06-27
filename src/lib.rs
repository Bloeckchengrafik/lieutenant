pub mod argument;
pub mod command;
mod generic;
pub mod parser;
pub mod regex;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use crate::command::builder::{literal, CommandBuilder};
    use crate::command::Command;

    #[test]
    fn simple() {
        // (Gamestate, Extract) -> Res    Extract -> (Gamestate -> Res)
        let command = literal("/").space().arg::<u32>();
        let x = command.on_call(|x| {
            move |game_state, _foo| {
                println!("hi {} the gamestate was {}", x, game_state);
                42
            }
        });

        let r = x.call((0, "test"), "/ 100 ").unwrap();
        assert!(r == 42);
    }

    #[test]
    fn one_optional_argument() {
        let command = literal("/test").opt_space().opt_arg::<u32>();
        let x = command.on_call(|_: Option<(u32,)>| move |_, _| 42);

        let suc = x.call((0, "test"), "/test 3");
        assert!(suc.is_ok(), "{:?}", suc);

        let suc = x.call((0, "test"), "/test");
        assert!(suc.is_ok(), "{:?}", suc);

        let err = x.call((0, "test"), "/test abc");
        assert!(err.is_err());
    }

    #[test]
    fn multiple_optional_argument() {
        let command = literal("/test")
            .opt_space()
            .opt_arg::<u32>()
            .opt_space()
            .opt_arg::<String>();
        let x = command.on_call(|_: Option<(u32,)>, _: Option<(String,)>| move |_, _| 42);

        x.call((0, "test"), "/test 3")
            .expect("This should only fill the first optional argument");
        x.call((0, "test"), "/test")
            .expect("This should fill no optional arguments");
        x.call((0, "test"), "/test abc")
            .expect("This should fill the second optional argument");
        assert!(x.call((0, "test"), "/test abc def").is_err());
    }
}
