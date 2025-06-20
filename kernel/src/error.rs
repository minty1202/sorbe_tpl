use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("token error: {0}")]
    Token(TokenError),
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
