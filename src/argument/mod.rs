mod choice;
mod numbers;
mod string_wildcard;
mod strings;
mod bool;

use crate::parser::IterParser;
pub use choice::*;
pub use numbers::*;
pub use string_wildcard::*;
pub use strings::*;

pub trait Argument {
    type Parser: IterParser<Extract = (Self,), ParserState = Self::ParserState> + Sized + Default;
    type ParserState: Default;
}
