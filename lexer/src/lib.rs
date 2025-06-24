mod peekable_extend;
mod sources;

use crate::peekable_extend::PeekableExt;
use kernel::{
    error::TokenError,
    source::{TokenResult, TryFromSource},
    token::Token,
    tokenize::Tokenize,
};

pub use sources::{config::ConfigSource, schema::SchemaSource};

pub struct Lexer;

impl Tokenize for Lexer {
    fn tokenize<T>(source: T) -> Result<Vec<Token>, TokenError>
    where
        T: TryFromSource + AsRef<str>,
    {
        let input = source.as_ref();
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if source.is_skip_char(c) {
                continue;
            }

            if source.is_invalid_chars(c) {
                return Err(TokenError::InvalidChar(c));
            }

            match source.classify_char(c) {
                TokenResult::Single(token) => tokens.push(token),
                TokenResult::NeedsBlock(start_char) => {
                    let token = source.process_block_token(start_char, &mut chars)?;
                    tokens.push(token);
                }
                TokenResult::Comment => {
                    let _ = chars.read_until_delimiter(|c| c == '\n', |_| None);
                    continue;
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
