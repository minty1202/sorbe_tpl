use crate::error::ParseError;
use crate::token::Token;
use crate::value::Value;

pub trait Parse {
    fn parse(&self, tokens: Vec<Token>) -> Result<Value, ParseError>;
}
