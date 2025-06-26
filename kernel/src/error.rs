use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("token error: {0}")]
    Lexer(#[from] TokenError),
    #[error("parser error: {0}")]
    Parse(#[from] ParseError),

    #[error("validation error: unknown key '{key}'")]
    UnknownKey { key: String },
    #[error("validation error: missing key '{key}'")]
    MissingKey { key: String },

    #[error("validation error: type mismatch: expected '{expected}', found '{found}'")]
    TypeMismatch { expected: String, found: String },

    #[error("serde error: {0}")]
    Serde(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
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
pub enum ParseError {
    #[error("Token validation error: {0}")]
    Token(#[from] TokenValidationError),
    #[error("Key validation error: {0}")]
    Syntax(#[from] SyntaxValidationError),
}

#[derive(Debug, Error)]
pub enum TokenValidationError {
    #[error("key validation error: {0}")]
    Key(#[from] KeyError),

    #[error("value validation error: {0}")]
    Value(#[from] ValueError),

    #[error("Line structure validation error: {0}")]
    LineStructure(#[from] LineStructureError),

    #[error("Internal validation error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum SyntaxValidationError {
    #[error("duplicate key found")]
    Duplicate { key: String },

    #[error("quoted value not allowed in this context")]
    QuotedNotAllowed,

    #[error("key path conflict: '{key}'")]
    KeyPathConflict { key: String },
}

#[derive(Debug, Error)]
pub enum KeyError {
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
pub enum ValueError {
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
pub enum LineStructureError {
    #[error("Missing separator in line")]
    MissingSeparators,

    #[error("Multiple separator found, expected exactly one")]
    MultipleSeparators,

    #[error("Missing key before '='")]
    MissingLeftSide,

    #[error("Token before '=' must be an identifier")]
    LeftSideMustBeIdent,

    #[error("Value contains invalid tokens")]
    RightSideContainsInvalidTokens,
}
