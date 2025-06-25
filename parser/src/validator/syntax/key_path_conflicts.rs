use super::SyntaxValidator;
use crate::syntax::Syntax;
use kernel::error::SyntaxValidationError;

impl SyntaxValidator {
    pub fn validate_key_path_conflicts(syntax: &Syntax) -> Result<(), SyntaxValidationError> {
        let paths: Vec<String> = syntax
            .patterns
            .iter()
            .map(|pattern| pattern.key_parts.join("."))
            .collect();

        for path in &paths {
            let has_children = paths
                .iter()
                .any(|other| other != path && other.starts_with(&format!("{}.", path)));

            if has_children {
                return Err(SyntaxValidationError::KeyPathConflict { key: path.clone() });
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
    fn test_valid_key_paths() {
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
        assert!(SyntaxValidator::validate_key_path_conflicts(&syntax).is_ok());

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
        assert!(SyntaxValidator::validate_key_path_conflicts(&syntax).is_ok());
    }

    #[test]
    fn test_invalid_key_paths() {
        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["key1".into()],
                    value: SyntaxValue::Plain("value1".into()),
                },
                Pattern {
                    key_parts: vec!["key1".into(), "subkey".into()],
                    value: SyntaxValue::Plain("value2".into()),
                },
            ],
        };
        assert!(SyntaxValidator::validate_key_path_conflicts(&syntax).is_err());

        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["key1".into()],
                    value: SyntaxValue::Plain("value1".into()),
                },
                Pattern {
                    key_parts: vec!["key1".into(), "subkey".into(), "subsubkey".into()],
                    value: SyntaxValue::Plain("value2".into()),
                },
            ],
        };
        assert!(SyntaxValidator::validate_key_path_conflicts(&syntax).is_err());
    }
}
