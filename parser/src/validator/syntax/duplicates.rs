use super::SyntaxValidator;
use crate::syntax::Syntax;
use kernel::error::SyntaxValidationError;
use std::collections::HashSet;

impl SyntaxValidator {
    pub fn validate_duplicate_keys(syntax: &Syntax) -> Result<(), SyntaxValidationError> {
        let mut seen_keys = HashSet::new();
        for pattern in &syntax.patterns {
            let key = pattern.key_parts.join(".");
            if !seen_keys.insert(key.clone()) {
                return Err(SyntaxValidationError::Duplicate { key });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{Pattern, Syntax, SyntaxValue};

    #[test]
    fn test_valid() {
        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["key1".into()],
                    value: SyntaxValue::Plain("value1".into()),
                },
                Pattern {
                    key_parts: vec!["key2".into()],
                    value: SyntaxValue::Plain("value2".into()),
                },
            ],
        };
        assert!(SyntaxValidator::validate_duplicate_keys(&syntax).is_ok());

        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["key1".into(), "subkey1".into()],
                    value: SyntaxValue::Plain("value1".into()),
                },
                Pattern {
                    key_parts: vec!["key1".into(), "subkey2".into()],
                    value: SyntaxValue::Plain("value2".into()),
                },
                Pattern {
                    key_parts: vec!["key2".into(), "subkey1".into()],
                    value: SyntaxValue::Plain("value3".into()),
                },
            ],
        };

        assert!(SyntaxValidator::validate_duplicate_keys(&syntax).is_ok());
    }

    mod invalid {
        use super::*;

        #[test]
        fn test_duplicate_keys() {
            let syntax = Syntax {
                patterns: vec![
                    Pattern {
                        key_parts: vec!["key1".into()],
                        value: SyntaxValue::Plain("value1".into()),
                    },
                    Pattern {
                        key_parts: vec!["key1".into()],
                        value: SyntaxValue::Plain("value2".into()),
                    },
                ],
            };
            assert!(matches!(
                SyntaxValidator::validate_duplicate_keys(&syntax),
                Err(SyntaxValidationError::Duplicate { key }) if key == "key1"
            ));

            let syntax = Syntax {
                patterns: vec![
                    Pattern {
                        key_parts: vec!["key1".into(), "subkey1".into()],
                        value: SyntaxValue::Plain("value1".into()),
                    },
                    Pattern {
                        key_parts: vec!["key1".into(), "subkey1".into()],
                        value: SyntaxValue::Plain("value2".into()),
                    },
                ],
            };
            assert!(matches!(
                SyntaxValidator::validate_duplicate_keys(&syntax),
                Err(SyntaxValidationError::Duplicate { key }) if key == "key1.subkey1"
            ));
        }
    }
}
