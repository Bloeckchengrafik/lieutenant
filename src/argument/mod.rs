mod numbers;
mod strings;
mod string_wildcard;

use crate::parser::IterParser;
pub use numbers::*;
pub use strings::*;
pub use string_wildcard::*;

pub trait Argument {
    type Parser: IterParser<Extract = (Self,), ParserState = Self::ParserState> + Sized + Default;
    type ParserState: Default;
}
