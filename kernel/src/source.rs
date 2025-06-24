use crate::error::TokenError;
use crate::token::Token;
use std::char;

use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum TokenResult {
    Single(Token),
    NeedsBlock(char),
    Comment,
}

pub trait TryFromSource {
    fn classify_char(&self, c: char) -> TokenResult;
    fn process_block_token(
        &self,
        char: char,
        chars: &mut Peekable<std::str::Chars>,
    ) -> Result<Token, TokenError>;

    fn is_skip_char(&self, c: char) -> bool {
        [' ', '\t', '\r'].contains(&c)
    }

    fn is_invalid_chars(&self, c: char) -> bool {
        const DEFAULT_INVALID: &[char] = &[
            '[', ']', '{', '}', ',', ';', '!', '@', '$', '%', '^', '&', '*', '(', ')', '+',
        ];

        DEFAULT_INVALID.contains(&c)
            || self.additional_invalid_chars().contains(&c)
            || !c.is_ascii()
    }

    fn additional_invalid_chars(&self) -> &[char] {
        &[]
    }
}
