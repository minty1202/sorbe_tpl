mod duplicates;
mod key_path_conflicts;

use crate::syntax::{Syntax, SyntaxValue};
use kernel::error::SyntaxValidationError;

pub trait ValidationRule {
    fn validate_value(value: &SyntaxValue) -> Result<(), SyntaxValidationError>;
}

pub struct ConfigRule;

impl ValidationRule for ConfigRule {
    fn validate_value(_value: &SyntaxValue) -> Result<(), SyntaxValidationError> {
        Ok(())
    }
}

pub struct SchemaRule;

impl ValidationRule for SchemaRule {
    fn validate_value(value: &SyntaxValue) -> Result<(), SyntaxValidationError> {
        match value {
            SyntaxValue::Quoted(_) => Err(SyntaxValidationError::QuotedNotAllowed),
            SyntaxValue::Plain(_) => Ok(()),
        }
    }
}

pub struct SyntaxValidator;

impl SyntaxValidator {
    pub fn validate<R: ValidationRule>(syntax: &Syntax) -> Result<(), SyntaxValidationError> {
        Self::validate_duplicate_keys(syntax)?;
        Self::validate_key_path_conflicts(syntax)?;

        for pattern in &syntax.patterns {
            R::validate_value(&pattern.value)?;
        }

        Ok(())
    }
}
