mod key;
mod line_structure;
mod value;

use crate::syntax::{Pattern, Syntax, SyntaxValue};
use kernel::error::TokenValidationError;
use kernel::token::Token;

pub struct TokenValidator;

impl TokenValidator {
    pub fn validate(tokens: Vec<Token>) -> Result<Syntax, TokenValidationError> {
        let token_lines: Vec<Vec<Token>> = tokens
            .split(|token| matches!(token, Token::Newline | Token::Eof))
            .map(|line| line.to_vec())
            .collect();

        let mut patterns = Vec::new();

        for line in token_lines {
            if line.is_empty() {
                continue;
            }

            if line
                .iter()
                .any(|token| matches!(token, Token::Newline | Token::Eof))
            {
                unreachable!("Lines containing Newline/Eof tokens should not be processed");
            };

            Self::validate_line_structure(&line)?;
            let (key_tokens, value_tokens) = Self::split_key_value(&line)?;
            Self::validate_key(&key_tokens)?;
            Self::validate_value(&value_tokens)?;
            let key = Self::build_key(&key_tokens);
            let value = Self::build_value(&value_tokens)?;
            patterns.push(Pattern {
                key_parts: key,
                value,
            });
        }

        Ok(Syntax { patterns })
    }

    fn split_key_value(tokens: &[Token]) -> Result<(Vec<Token>, Vec<Token>), TokenValidationError> {
        let mut key_tokens = Vec::new();
        let mut value_tokens = Vec::new();

        let equal_pos = tokens.iter().position(|t| matches!(t, Token::Separator));

        match equal_pos {
            Some(pos) => {
                key_tokens.extend_from_slice(&tokens[..pos]);
                value_tokens.extend_from_slice(&tokens[pos + 1..]);
            }
            None => {
                unreachable!("There should be exactly one Separator token at this point");
            }
        }

        Ok((key_tokens, value_tokens))
    }

    fn build_key(key_tokens: &[Token]) -> Vec<String> {
        key_tokens
            .iter()
            .filter_map(|token| match token {
                Token::Ident(name) => Some(name.clone()),
                _ => None,
            })
            .collect()
    }

    fn build_value(value_tokens: &[Token]) -> Result<SyntaxValue, TokenValidationError> {
        if value_tokens.is_empty() {
            return Ok(SyntaxValue::Plain(String::new()));
        }

        if value_tokens.len() == 1 {
            match &value_tokens[0] {
                Token::QuotedIdent(name) => return Ok(SyntaxValue::Quoted(name.clone())),
                Token::Ident(name) => return Ok(SyntaxValue::Plain(name.clone())),
                _ => {}
            }
        }

        let mut result = String::new();
        for token in value_tokens {
            match token {
                Token::Ident(name) => result.push_str(name),
                Token::Dot => result.push('.'),
                _ => {
                    return Err(TokenValidationError::Internal(
                        "Unexpected token in value building".to_string(),
                    ));
                }
            }
        }

        Ok(SyntaxValue::Plain(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernel::token::Token;

    #[test]
    fn test_syntax_validator() {
        let tokens = vec![
            Token::Ident("key1".to_string()),
            Token::Separator,
            Token::Ident("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Separator,
            Token::QuotedIdent("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ];

        let syntax = TokenValidator::validate(tokens).unwrap();

        assert_eq!(syntax.patterns.len(), 2);
        assert_eq!(syntax.patterns[0].key_parts, vec!["key1".to_string()]);
        assert_eq!(
            syntax.patterns[0].value,
            SyntaxValue::Plain("value1".to_string())
        );
        assert_eq!(syntax.patterns[1].key_parts, vec!["key2".to_string()]);
        assert_eq!(
            syntax.patterns[1].value,
            SyntaxValue::Quoted("value2".to_string())
        );

        let tokens: Vec<Token> = vec![Token::Eof];
        let empty_syntax = TokenValidator::validate(tokens).unwrap();
        assert!(empty_syntax.patterns.is_empty());

        let tokens = vec![
            Token::Ident("key1".to_string()),
            Token::Dot,
            Token::Ident("subkey".to_string()),
            Token::Separator,
            Token::Ident("value1".to_string()),
            Token::Newline,
        ];
        let syntax = TokenValidator::validate(tokens).unwrap();
        assert_eq!(syntax.patterns.len(), 1);
        assert_eq!(
            syntax.patterns[0].key_parts,
            vec!["key1".to_string(), "subkey".to_string()]
        );
        assert_eq!(
            syntax.patterns[0].value,
            SyntaxValue::Plain("value1".to_string())
        );

        let tokens = vec![
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value1".to_string()),
            Token::Newline,
        ];
        let syntax = TokenValidator::validate(tokens).unwrap();
        assert_eq!(syntax.patterns.len(), 1);
        assert_eq!(syntax.patterns[0].key_parts, vec!["key".to_string()]);
        assert_eq!(
            syntax.patterns[0].value,
            SyntaxValue::Quoted("value1".to_string())
        );

        let tokens = vec![
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("0".to_string()),
            Token::Dot,
            Token::Ident("1".to_string()),
            Token::Newline,
        ];

        let syntax = TokenValidator::validate(tokens).unwrap();
        assert_eq!(syntax.patterns.len(), 1);
        assert_eq!(syntax.patterns[0].key_parts, vec!["key".to_string()]);
        assert_eq!(
            syntax.patterns[0].value,
            SyntaxValue::Plain("0.1".to_string())
        );

        let tokens = vec![
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Dot,
            Token::Ident("1".to_string()),
            Token::Newline,
        ];
        let syntax = TokenValidator::validate(tokens).unwrap();
        assert_eq!(syntax.patterns.len(), 1);
        assert_eq!(syntax.patterns[0].key_parts, vec!["key".to_string()]);
        assert_eq!(
            syntax.patterns[0].value,
            SyntaxValue::Plain(".1".to_string())
        );

        let tokens = vec![
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Newline,
        ];
        let syntax = TokenValidator::validate(tokens).unwrap();
        assert_eq!(syntax.patterns.len(), 1);
        assert_eq!(syntax.patterns[0].key_parts, vec!["key".to_string()]);
        assert_eq!(syntax.patterns[0].value, SyntaxValue::Plain("".to_string()));

        let tokens = vec![
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("-1".to_string()),
            Token::Newline,
        ];
        let syntax = TokenValidator::validate(tokens).unwrap();
        assert_eq!(syntax.patterns.len(), 1);
        assert_eq!(syntax.patterns[0].key_parts, vec!["key".to_string()]);
        assert_eq!(
            syntax.patterns[0].value,
            SyntaxValue::Plain("-1".to_string())
        );
    }

    #[test]
    fn test_invalid_syntax() {
        let invalid_tokens = vec![Token::Ident("invalid".to_string()), Token::Newline];
        assert!(TokenValidator::validate(invalid_tokens).is_err());
    }
}
