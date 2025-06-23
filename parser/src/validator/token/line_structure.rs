use super::TokenValidator;
use SyntaxLineStructureValidationError as LineError;
use SyntaxValidationError as BaseError;
use Token::*;
use kernel::error::{SyntaxLineStructureValidationError, SyntaxValidationError};
use kernel::token::Token;

impl TokenValidator {
    pub fn validate_line_structure(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        Self::validate_equal_count(tokens)?;

        let (left_side, right_side) = Self::split_side_by_side(tokens)?;

        Self::validate_left_side_last(left_side)?;
        Self::validate_right_side_last(right_side)?;

        Ok(())
    }

    fn validate_equal_count(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        let equal_count = tokens.iter().filter(|t| matches!(t, Token::Equal)).count();
        if equal_count == 0 {
            return Err(BaseError::LineStructure(LineError::MissingEquals));
        } else if equal_count > 1 {
            return Err(BaseError::LineStructure(LineError::MultipleEquals));
        }

        Ok(())
    }

    fn validate_left_side_last(left_side: &[Token]) -> Result<(), SyntaxValidationError> {
        match left_side.last() {
            Some(Ident(_)) => {}
            Some(_) => return Err(BaseError::LineStructure(LineError::LeftSideMustBeIdent)),
            None => return Err(BaseError::LineStructure(LineError::MissingLeftSide)),
        }
        Ok(())
    }

    fn validate_right_side_last(right_side: &[Token]) -> Result<(), SyntaxValidationError> {
        match right_side.last() {
            Some(Ident(_) | QuotedIdent(_)) => {}
            None => {}
            Some(_) => {
                return Err(BaseError::LineStructure(
                    LineError::RightSideContainsInvalidTokens,
                ));
            }
        }
        Ok(())
    }

    fn split_side_by_side(tokens: &[Token]) -> Result<(&[Token], &[Token]), SyntaxValidationError> {
        let equal_pos = tokens
            .iter()
            .position(|t| matches!(t, Token::Equal))
            .unwrap_or_else(|| {
                unreachable!("Equal token must be validated before calling this function")
            });

        Ok((&tokens[..equal_pos], &tokens[equal_pos + 1..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use SyntaxLineStructureValidationError as LE;
    use SyntaxValidationError::LineStructure as LineStructureError;
    use kernel::error::{SyntaxLineStructureValidationError, SyntaxValidationError};

    #[test]
    fn test_valid() {
        let tokens = vec![Ident("key".to_string()), Equal, Ident("value".to_string())];
        assert!(TokenValidator::validate_line_structure(&tokens).is_ok());

        let tokens = vec![
            Ident("key".to_string()),
            Dot,
            Ident("subkey".to_string()),
            Equal,
            Ident("value".to_string()),
        ];

        assert!(TokenValidator::validate_line_structure(&tokens).is_ok());

        let tokens = vec![
            Ident("key".to_string()),
            Equal,
            QuotedIdent("value".to_string()),
        ];
        assert!(TokenValidator::validate_line_structure(&tokens).is_ok());
    }

    mod invalid {
        use super::*;

        #[test]
        fn test_missing_equal() {
            let tokens = vec![Ident("key".to_string()), Ident("value".to_string())];
            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::MissingEquals)
            ));
        }

        #[test]
        fn test_multiple_equals() {
            let tokens = vec![
                Ident("key".to_string()),
                Equal,
                Ident("value1".to_string()),
                Equal,
                Ident("value2".to_string()),
            ];
            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::MultipleEquals)
            ));

            let tokens = vec![
                Ident("key".to_string()),
                Equal,
                Equal,
                Ident("value".to_string()),
            ];

            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::MultipleEquals)
            ));
        }

        #[test]
        fn test_missing_left_side() {
            let tokens = vec![Equal, Ident("value".to_string()), Newline];
            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::MissingLeftSide)
            ));
        }

        #[test]
        fn test_left_side_must_be_ident() {
            let tokens = vec![
                QuotedIdent("key".to_string()),
                Equal,
                Ident("value".to_string()),
                Newline,
            ];
            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::LeftSideMustBeIdent)
            ));

            let tokens = vec![Dot, Equal, Ident("value".to_string()), Newline];
            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::LeftSideMustBeIdent)
            ));
        }

        #[test]
        fn test_right_side_contains_invalid_tokens() {
            let tokens = vec![Ident("key".to_string()), Equal, Dot, Newline];
            let result = TokenValidator::validate_line_structure(&tokens);
            assert!(matches!(
                result.unwrap_err(),
                LineStructureError(LE::RightSideContainsInvalidTokens)
            ));
        }
    }
}
