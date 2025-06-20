use crate::error::TokenError;
use crate::token::Token;

pub trait Tokenize {
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, TokenError>;
}
