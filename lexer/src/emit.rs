use super::peekable_extend::PeekableExt;
use super::{BlockToken, SingleToken, SymbolChar};

use kernel::{error::TokenError, token::Token};

use std::iter::Peekable;

impl SymbolChar {
    pub fn emit_token(
        &self,
        chars: &mut Peekable<std::str::Chars>,
    ) -> Result<Option<Token>, TokenError> {
        let token = match self {
            SymbolChar::Single(single) => Some(single.to_token()),
            SymbolChar::Block(BlockToken::Ident(first_char)) => {
                let (equal, dot) = SingleToken::chars();

                let content = chars
                    .read_until_delimiter(
                        |c| c == ' ' || c == equal || c == dot || c == '\n',
                        |c| Some(Self::is_invalid_chars(c)),
                    )
                    .map_err(TokenError::InvalidChar)?;

                let with_first_char = format!("{}{}", first_char, content);

                Some(Token::Ident(with_first_char))
            }
            SymbolChar::Block(BlockToken::SingleQuoteIdent) => {
                let content = chars
                    .read_until_terminator(
                        |c| c == BlockToken::SingleQuoteIdent.as_char(),
                        |c| Some(c.is_control()),
                    )
                    .map_err(|_| TokenError::UnterminatedString)?;

                Some(Token::QuotedIdent(content))
            }
            SymbolChar::Block(BlockToken::DoubleQuoteIdent) => {
                let content = chars
                    .read_until_terminator(
                        |c| c == BlockToken::DoubleQuoteIdent.as_char(),
                        |c| Some(c.is_control()),
                    )
                    .map_err(|_| TokenError::UnterminatedString)?;

                let escaped = Self::process_escape_sequences(content);
                Some(Token::QuotedIdent(escaped))
            }
            SymbolChar::Block(BlockToken::Comment) => {
                chars
                    .read_until_delimiter(|c| c == '\n', |_| None)
                    .unwrap_or_default();

                None
            }
        };
        Ok(token)
    }

    fn process_escape_sequences(input: String) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('\'') => result.push('\''),
                    Some('0') => result.push('\0'),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => {
                        result.push('\\');
                    }
                }
            } else {
                result.push(c);
            }
        }

        result
    }
}
