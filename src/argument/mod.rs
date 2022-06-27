mod numbers;
mod string_wildcard;
mod strings;

use crate::parser::IterParser;
pub use numbers::*;
pub use string_wildcard::*;
pub use strings::*;

pub trait Argument {
    type Parser: IterParser<Extract = (Self,), ParserState = Self::ParserState> + Sized + Default;
    type ParserState: Default;
}
