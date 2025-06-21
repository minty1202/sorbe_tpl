use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("token error: {0}")]
    Token(TokenError),
    #[error("Syntax validation error: {0}")]
    SyntaxValidation(SyntaxValidationError),
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Invalid character: '{0}'")]
    InvalidChar(char),
    #[error("Unterminated string literal")]
    UnterminatedString,
    #[error("Internal lexer error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum SyntaxValidationError {
    #[error("key validation error: {0}")]
    Key(#[from] SyntaxKeyValidationError),

    #[error("value validation error: {0}")]
    Value(#[from] SyntaxValueValidationError),

    #[error("Line structure validation error: {0}")]
    LineStructure(#[from] SyntaxLineStructureValidationError),

    #[error("Internal validation error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum SyntaxKeyValidationError {
    #[error("Key cannot start with hyphen: '{key_part}'")]
    InvalidKeyStartsWithHyphen { key_part: String },

    #[error("Key cannot end with hyphen: '{key_part}'")]
    InvalidKeyEndsWithHyphen { key_part: String },

    #[error("Unexpected token in key")]
    UnexpectedTokenInKey,

    #[error("Key cannot be numeric: '{key_part}'")]
    KeyCannotBeNumeric { key_part: String },
}

#[derive(Debug, Error)]
pub enum SyntaxValueValidationError {
    #[error("Value cannot contain multiple non-numeric identifiers")]
    MultipleNonNumericIdents,

    #[error("Value cannot contain multiple quoted identifiers")]
    MultipleQuotedIdents,

    #[error("Value cannot contain multiple mixed identifiers")]
    MultipleMixedIdents,

    #[error("Value cannot contain multiple dots")]
    MultipleDots,

    #[error("Invalid value format")]
    InvalidValueFormat,
}

#[derive(Debug, Error)]
pub enum SyntaxLineStructureValidationError {
    #[error("Missing '=' in line")]
    MissingEquals,

    #[error("Multiple '=' found, expected exactly one")]
    MultipleEquals,

    #[error("Missing key before '='")]
    MissingLeftSide,

    #[error("Token before '=' must be an identifier")]
    LeftSideMustBeIdent,

    #[error("Value contains invalid tokens")]
    RightSideContainsInvalidTokens,
}
