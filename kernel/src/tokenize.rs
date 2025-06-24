use crate::error::TokenError;
use crate::source::TryFromSource;
use crate::token::Token;
use std::convert::AsRef;

pub trait Tokenize {
    fn tokenize<T>(source: T) -> Result<Vec<Token>, TokenError>
    where
        T: TryFromSource + AsRef<str>;
}
