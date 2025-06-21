use super::SyntaxValidator;
use crate::error::{SyntaxValidationError, SyntaxValueValidationError};
use crate::token::Token;
use From;
use SyntaxValidationError as BaseError;
use SyntaxValueValidationError as ValueError;
use Token::*;

enum InnerPlaneToken {
    String,
    Numeric,
    Dot,
}

impl From<&Token> for InnerPlaneToken {
    fn from(token: &Token) -> Self {
        match token {
            Ident(name) if name.chars().all(|c| c.is_numeric()) => InnerPlaneToken::Numeric,
            Ident(_) => InnerPlaneToken::String,
            Dot => InnerPlaneToken::Dot,
            _ => unreachable!(
                "Only Ident or Dot should be passed to InnerPlaneToken::from; got: {:?}",
                token
            ),
        }
    }
}

impl SyntaxValidator {
    pub fn validate_value(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        if tokens.is_empty() {
            return Ok(());
        }

        if tokens.len() == 1 {
            return Self::validate_single_value(tokens[0].clone());
        }

        Self::validate_multiple_quoted_idents(tokens)?;
        Self::validate_mixed_idents(tokens)?;
        Self::validate_multiple_dots(tokens)?;
        Self::validate_ending_dot(tokens)?;

        let inner_simple_tokens: Vec<InnerPlaneToken> =
            tokens.iter().map(InnerPlaneToken::from).collect();

        Self::validate_multiple_non_numeric_idents(&inner_simple_tokens)?;
        Self::validate_mixed_dot_and_non_numeric_idents(&inner_simple_tokens)?;
        Self::validate_mixed_non_numeric_and_numeric_idents(&inner_simple_tokens)?;
        Self::validate_consecutive_numeric_idents(&inner_simple_tokens)?;

        Ok(())
    }

    fn validate_single_value(token: Token) -> Result<(), SyntaxValidationError> {
        if token == Dot {
            return Err(BaseError::Value(ValueError::InvalidValueFormat));
        }
        Ok(())
    }

    fn validate_mixed_idents(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        let has_simple_ident = tokens.iter().any(|t| matches!(t, Ident(_)));
        let has_quoted_ident = tokens.iter().any(|t| matches!(t, QuotedIdent(_)));
        if has_simple_ident && has_quoted_ident {
            return Err(BaseError::Value(ValueError::MultipleMixedIdents));
        }

        Ok(())
    }

    fn validate_mixed_dot_and_non_numeric_idents(
        inner_simple_tokens: &[InnerPlaneToken],
    ) -> Result<(), SyntaxValidationError> {
        let has_dot = inner_simple_tokens
            .iter()
            .any(|c| matches!(c, InnerPlaneToken::Dot));
        let has_non_numeric = inner_simple_tokens
            .iter()
            .any(|c| matches!(c, InnerPlaneToken::String));

        if has_dot && has_non_numeric {
            return Err(BaseError::Value(ValueError::InvalidValueFormat));
        }

        Ok(())
    }

    fn validate_mixed_non_numeric_and_numeric_idents(
        inner_simple_tokens: &[InnerPlaneToken],
    ) -> Result<(), SyntaxValidationError> {
        let has_numeric = inner_simple_tokens
            .iter()
            .any(|c| matches!(c, InnerPlaneToken::Numeric));
        let has_non_numeric = inner_simple_tokens
            .iter()
            .any(|c| matches!(c, InnerPlaneToken::String));

        if has_numeric && has_non_numeric {
            return Err(BaseError::Value(ValueError::InvalidValueFormat));
        }

        Ok(())
    }

    fn validate_multiple_quoted_idents(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        let quoted_ident_count = tokens
            .iter()
            .filter(|t| matches!(t, QuotedIdent(_)))
            .count();
        if quoted_ident_count > 1 {
            return Err(BaseError::Value(ValueError::MultipleQuotedIdents));
        }

        Ok(())
    }

    fn validate_multiple_dots(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        let dot_count = tokens.iter().filter(|t| matches!(t, Dot)).count();
        if dot_count > 1 {
            return Err(BaseError::Value(ValueError::MultipleDots));
        }

        Ok(())
    }

    fn validate_multiple_non_numeric_idents(
        inner_simple_tokens: &[InnerPlaneToken],
    ) -> Result<(), SyntaxValidationError> {
        let simple_string_idents_count = inner_simple_tokens
            .iter()
            .filter(|c| matches!(c, InnerPlaneToken::String))
            .count();
        if simple_string_idents_count > 1 {
            return Err(BaseError::Value(ValueError::MultipleNonNumericIdents));
        }

        Ok(())
    }

    fn validate_ending_dot(tokens: &[Token]) -> Result<(), SyntaxValidationError> {
        if tokens.last() == Some(&Dot) {
            return Err(BaseError::Value(ValueError::InvalidValueFormat));
        }
        Ok(())
    }

    fn validate_consecutive_numeric_idents(
        inner_simple_tokens: &[InnerPlaneToken],
    ) -> Result<(), SyntaxValidationError> {
        for i in 0..inner_simple_tokens.len() {
            if matches!(inner_simple_tokens[i], InnerPlaneToken::Numeric) {
                if let Some(InnerPlaneToken::Numeric) = inner_simple_tokens.get(i + 1) {
                    return Err(BaseError::Value(ValueError::InvalidValueFormat));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{SyntaxValidationError, SyntaxValueValidationError};
    use SyntaxValidationError::Value as ValueError;
    use SyntaxValueValidationError as VE;

    #[test]
    fn test_valid() {
        let tokens = vec![Ident("value".to_string())];
        assert!(SyntaxValidator::validate_value(&tokens).is_ok());

        let tokens = vec![QuotedIdent("value".to_string())];
        assert!(SyntaxValidator::validate_value(&tokens).is_ok());

        let tokens = vec![Ident("123".to_string())];
        assert!(SyntaxValidator::validate_value(&tokens).is_ok());

        let tokens = vec![Ident("123".to_string()), Dot, Ident("456".to_string())];
        assert!(SyntaxValidator::validate_value(&tokens).is_ok());

        let tokens = vec![Dot, Ident("123".to_string())];
        assert!(SyntaxValidator::validate_value(&tokens).is_ok());

        let tokens: Vec<Token> = vec![];
        assert!(SyntaxValidator::validate_value(&tokens).is_ok());
    }

    mod invalid {
        use super::*;

        #[test]
        fn test_multiple_non_numeric_idents() {
            let tokens = vec![Ident("value1".to_string()), Ident("value2".to_string())];
            let result = SyntaxValidator::validate_value(&tokens);
            assert!(matches!(
                result,
                Err(ValueError(VE::MultipleNonNumericIdents))
            ));
        }

        #[test]
        fn test_multiple_quoted_idents() {
            let tokens = vec![
                QuotedIdent("value1".to_string()),
                QuotedIdent("value2".to_string()),
            ];
            let result = SyntaxValidator::validate_value(&tokens);
            assert!(matches!(result, Err(ValueError(VE::MultipleQuotedIdents))));
        }

        #[test]
        fn test_multiple_mixed_idents() {
            let tokens = vec![
                Ident("value1".to_string()),
                QuotedIdent("value2".to_string()),
            ];
            let result = SyntaxValidator::validate_value(&tokens);
            assert!(matches!(result, Err(ValueError(VE::MultipleMixedIdents))));

            let tokens = vec![Ident("123".to_string()), QuotedIdent("value2".to_string())];
            let result = SyntaxValidator::validate_value(&tokens);
            assert!(matches!(result, Err(ValueError(VE::MultipleMixedIdents))));

            let tokens = vec![QuotedIdent("value2".to_string()), Ident("123".to_string())];
            let result = SyntaxValidator::validate_value(&tokens);
            assert!(matches!(result, Err(ValueError(VE::MultipleMixedIdents))));

            let tokens = vec![
                QuotedIdent("value2".to_string()),
                Ident("value1".to_string()),
            ];
            let result = SyntaxValidator::validate_value(&tokens);
            assert!(matches!(result, Err(ValueError(VE::MultipleMixedIdents))));
        }
    }

    #[test]
    fn test_multiple_dots() {
        let tokens = vec![
            Ident("123".to_string()),
            Dot,
            Ident("456".to_string()),
            Dot,
            Ident("789".to_string()),
        ];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::MultipleDots))));

        let tokens = vec![Ident("123".to_string()), Dot, Dot, Ident("789".to_string())];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::MultipleDots))));

        let tokens = vec![Dot, Dot, Ident("789".to_string())];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::MultipleDots))));

        let tokens = vec![Dot, Dot];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::MultipleDots))));
    }

    #[test]
    fn test_invalid_value_format() {
        let tokens = vec![Dot];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::InvalidValueFormat))));

        let tokens = vec![Ident("123".to_string()), Dot];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::InvalidValueFormat))));

        let tokens = vec![Ident("123".to_string()), Ident("aaa".to_string())];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::InvalidValueFormat))));

        let tokens = vec![Ident("123".to_string()), Ident("456".to_string())];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::InvalidValueFormat))));

        let tokens = vec![
            Ident("123".to_string()),
            Dot,
            Ident("456".to_string()),
            Ident("789".to_string()),
        ];
        let result = SyntaxValidator::validate_value(&tokens);
        assert!(matches!(result, Err(ValueError(VE::InvalidValueFormat))));
    }
}
