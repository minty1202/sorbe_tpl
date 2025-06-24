use kernel::{
    error::TokenError,
    source::{TokenResult, TryFromSource},
    token::Token,
};

use crate::peekable_extend::PeekableExt;

use std::convert::AsRef;
use std::iter::Peekable;

pub struct ConfigSource {
    pub input: String,
}

impl AsRef<str> for ConfigSource {
    fn as_ref(&self) -> &str {
        &self.input
    }
}

impl ConfigSource {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn process_escape_sequences(&self, input: String) -> String {
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

impl TryFromSource for ConfigSource {
    fn classify_char(&self, c: char) -> TokenResult {
        match c {
            '=' => TokenResult::Single(Token::Separator),
            '.' => TokenResult::Single(Token::Dot),
            '\n' => TokenResult::Single(Token::Newline),

            '"' => TokenResult::NeedsBlock(c),
            '\'' => TokenResult::NeedsBlock(c),
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
        match start_char {
            '"' => {
                let content = chars
                    .read_until_terminator(|c| c == '"', |c| Some(c.is_control()))
                    .map_err(|_| TokenError::UnterminatedString)?;

                let escaped = self.process_escape_sequences(content);
                Ok(Token::QuotedIdent(escaped))
            }
            '\'' => {
                let content = chars
                    .read_until_terminator(|c| c == '\'', |c| Some(c.is_control()))
                    .map_err(|_| TokenError::UnterminatedString)?;

                Ok(Token::QuotedIdent(content))
            }
            _ => {
                let content = chars
                    .read_until_delimiter(
                        |c| c == ' ' || c == '.' || c == '=' || c == '\n',
                        |c| Some(self.is_invalid_chars(c)),
                    )
                    .map_err(TokenError::InvalidChar)?;

                let with_start_char = format!("{}{}", start_char, content);

                Ok(Token::Ident(with_start_char))
            }
        }
    }

    fn additional_invalid_chars(&self) -> &[char] {
        &[';']
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::source::TokenResult;
    use kernel::token::Token;

    mod classify_char {
        use super::*;

        #[test]
        fn test_classify_char() {
            let source = ConfigSource::new("".to_string());

            assert_eq!(
                source.classify_char('='),
                TokenResult::Single(Token::Separator)
            );
            assert_eq!(source.classify_char('.'), TokenResult::Single(Token::Dot));
            assert_eq!(
                source.classify_char('\n'),
                TokenResult::Single(Token::Newline)
            );
            assert_eq!(source.classify_char('"'), TokenResult::NeedsBlock('"'));
            assert_eq!(source.classify_char('\''), TokenResult::NeedsBlock('\''));
            assert_eq!(source.classify_char('#'), TokenResult::Comment);
            assert_eq!(source.classify_char('a'), TokenResult::NeedsBlock('a'));
            assert_eq!(source.classify_char('_'), TokenResult::NeedsBlock('_'));
        }

        #[test]
        #[should_panic]
        fn test_classify_char_ja() {
            let source = ConfigSource::new("".to_string());
            source.classify_char('あ');
        }
    }

    mod process_block_token {
        use super::*;

        #[test]
        fn test_process_block_token() {
            let source = ConfigSource::new("".to_string());
            let mut chars = "\"Hello World\"".chars().peekable();
            chars.next();

            let token = source.process_block_token('"', &mut chars).unwrap();
            assert_eq!(token, Token::QuotedIdent("Hello World".to_string()));

            let mut chars = r#""Hello\nWorld""#.chars().peekable();
            chars.next();
            let token = source.process_block_token('"', &mut chars).unwrap();
            assert_eq!(token, Token::QuotedIdent("Hello\nWorld".to_string()));

            let mut chars = "'Hello World'".chars().peekable();
            chars.next();
            let token = source.process_block_token('\'', &mut chars).unwrap();
            assert_eq!(token, Token::QuotedIdent("Hello World".to_string()));

            let mut chars = r#"'Hello\nWorld'"#.chars().peekable();
            chars.next();
            let token = source.process_block_token('\'', &mut chars).unwrap();
            assert_eq!(token, Token::QuotedIdent("Hello\\nWorld".to_string()));

            let mut chars = "Identifier".chars().peekable();
            chars.next();
            let token = source.process_block_token('I', &mut chars).unwrap();
            assert_eq!(token, Token::Ident("Identifier".to_string()));

            let mut chars = "Identifier with space".chars().peekable();
            chars.next();
            let token = source.process_block_token('I', &mut chars).unwrap();
            assert_eq!(token, Token::Ident("Identifier".to_string()));

            let mut chars = "Identifier.with.dot".chars().peekable();
            chars.next();
            let token = source.process_block_token('I', &mut chars).unwrap();
            assert_eq!(token, Token::Ident("Identifier".to_string()));

            let mut chars = "Identifier=with=equals".chars().peekable();
            chars.next();
            let token = source.process_block_token('I', &mut chars).unwrap();
            assert_eq!(token, Token::Ident("Identifier".to_string()));

            let mut chars = "# This is a comment".chars().peekable();
            chars.next();
        }
    }
}
