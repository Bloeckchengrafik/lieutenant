mod numbers;
mod strings;

use crate::parser::IterParser;
pub use numbers::*;
pub use strings::*;

pub trait Argument {
    type Parser: IterParser<Extract = (Self,), ParserState = Self::ParserState> + Sized + Default;
    type ParserState: Default;
}
