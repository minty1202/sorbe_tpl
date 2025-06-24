use kernel::{
    error::TokenError,
    source::{TokenResult, TryFromSource},
    token::Token,
};

use crate::peekable_extend::PeekableExt;

use std::convert::AsRef;
use std::iter::Peekable;

pub struct SchemaSource {
    pub input: String,
}

impl AsRef<str> for SchemaSource {
    fn as_ref(&self) -> &str {
        &self.input
    }
}

impl SchemaSource {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl TryFromSource for SchemaSource {
    fn classify_char(&self, c: char) -> TokenResult {
        match c {
            ':' => TokenResult::Single(Token::Separator),
            '.' => TokenResult::Single(Token::Dot),
            '\n' => TokenResult::Single(Token::Newline),

            '#' => TokenResult::Comment,
            _ if c.is_ascii() || c == '_' => TokenResult::NeedsBlock(c),
            _ => unreachable!("Unexpected character: {}", c),
        }
    }

    fn process_block_token(
        &self,
        start_char: char,
        chars: &mut Peekable<std::str::Chars>,
    ) -> Result<Token, TokenError> {
        let content = chars
            .read_until_delimiter(
                |c| c == ' ' || c == '.' || c == ':' || c == '\n',
                |c| Some(self.is_invalid_chars(c)),
            )
            .map_err(TokenError::InvalidChar)?;

        let with_start_char = format!("{}{}", start_char, content);
        Ok(Token::Ident(with_start_char))
    }

    fn additional_invalid_chars(&self) -> &[char] {
        &['=', '"', '\'']
    }
}
