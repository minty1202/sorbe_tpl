use crate::error::ParseError;
use crate::token::Token;

pub trait Parse<T> {
    fn parse(tokens: Vec<Token>) -> Result<T, ParseError>;
}
