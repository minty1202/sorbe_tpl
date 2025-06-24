mod duplicates;

use crate::syntax::Syntax;
use kernel::error::SyntaxValidationError;

pub struct SyntaxValidator;

impl SyntaxValidator {
    pub fn validate(syntax: &Syntax) -> Result<(), SyntaxValidationError> {
        Self::validate_duplicate_keys(syntax)?;
        Ok(())
    }
}
