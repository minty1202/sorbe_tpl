use super::SyntaxValidator;
use crate::error::{SyntaxKeyValidationError, SyntaxValidationError};
use crate::token::Token;

use SyntaxKeyValidationError as KeyError;
use SyntaxValidationError as BaseError;
use Token::*;

impl SyntaxValidator {
    pub fn validate_key(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        let mut iter = tokens.iter();
        loop {
            let key_part = match iter.next() {
                Some(Ident(name)) => name,
                _ => {
                    return Err(BaseError::Key(KeyError::UnexpectedTokenInKey));
                }
            };

            Self::validate_start_with_hyphen(key_part)?;
            Self::validate_ends_with_hyphen(key_part)?;
            Self::validate_first_char_is_not_numeric(key_part)?;

            match iter.next() {
                Some(Dot) => continue,
                None => break,
                _ => {
                    return Err(BaseError::Key(KeyError::UnexpectedTokenInKey));
                }
            }
        }

        Ok(())
    }

    fn validate_start_with_hyphen(key_part: &str) -> Result<(), SyntaxValidationError> {
        if key_part.starts_with('-') {
            return Err(BaseError::Key(KeyError::InvalidKeyStartsWithHyphen {
                key_part: key_part.to_string(),
            }));
        }
        Ok(())
    }

    fn validate_ends_with_hyphen(key_part: &str) -> Result<(), SyntaxValidationError> {
        if key_part.ends_with('-') {
            return Err(BaseError::Key(KeyError::InvalidKeyEndsWithHyphen {
                key_part: key_part.to_string(),
            }));
        }
        Ok(())
    }

    fn validate_first_char_is_not_numeric(key_part: &str) -> Result<(), SyntaxValidationError> {
        if key_part.chars().next().is_some_and(|c| c.is_numeric()) {
            return Err(BaseError::Key(KeyError::KeyCannotBeNumeric {
                key_part: key_part.to_string(),
            }));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{SyntaxKeyValidationError, SyntaxValidationError};
    use SyntaxKeyValidationError as KE;
    use SyntaxValidationError::Key as KeyError;

    #[test]
    fn test_valid() {
        let tokens = vec![Ident("key".to_string())];
        assert!(SyntaxValidator::validate_key(&tokens).is_ok());

        let tokens_with_hyphen = vec![Ident("key-subkey".to_string())];
        assert!(SyntaxValidator::validate_key(&tokens_with_hyphen).is_ok());

        let tokens_with_numeric = vec![Ident("key123".to_string())];
        assert!(SyntaxValidator::validate_key(&tokens_with_numeric).is_ok());

        let tokens_with_dot = vec![Ident("key".to_string()), Dot, Ident("subkey".to_string())];
        assert!(SyntaxValidator::validate_key(&tokens_with_dot).is_ok());

        let tokens_with_multiple_dots = vec![
            Ident("key".to_string()),
            Dot,
            Ident("subkey".to_string()),
            Dot,
            Ident("subsubkey".to_string()),
        ];
        assert!(SyntaxValidator::validate_key(&tokens_with_multiple_dots).is_ok());
    }

    mod invalid {
        use super::*;

        #[test]
        fn test_key_starts_with_hyphen() {
            let tokens = vec![Ident("-key".to_string())];
            let result = SyntaxValidator::validate_key(&tokens);
            assert!(matches!(
                result,
                Err(KeyError(KE::InvalidKeyStartsWithHyphen { .. }))
            ));
        }

        #[test]
        fn test_key_with_trailing_hyphen() {
            let tokens = vec![Ident("key-".to_string())];
            let result = SyntaxValidator::validate_key(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                KeyError(KE::InvalidKeyEndsWithHyphen { .. })
            ));
        }

        #[test]
        fn test_dot_after_dot() {
            let tokens = vec![Ident("key".to_string()), Dot, Dot];
            let result = SyntaxValidator::validate_key(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                KeyError(KE::UnexpectedTokenInKey)
            ));
        }

        #[test]
        fn test_unexpected_token_in_key() {
            let tokens = vec![Ident("key".to_string()), Ident("subkey".to_string())];
            let result = SyntaxValidator::validate_key(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                KeyError(KE::UnexpectedTokenInKey)
            ));
        }

        #[test]
        fn test_key_with_trailing_dot() {
            let tokens = vec![Ident("key".to_string()), Dot];
            let result = SyntaxValidator::validate_key(&tokens);
            assert!(result.is_err());
            assert!(matches!(
                result.unwrap_err(),
                KeyError(KE::UnexpectedTokenInKey)
            ));
        }

        #[test]
        fn test_numeric_key() {
            let tokens = vec![Ident("123".to_string())];
            let result = SyntaxValidator::validate_key(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                KeyError(KE::KeyCannotBeNumeric { key_part })
                if key_part == "123"
            ));
        }
    }
}
