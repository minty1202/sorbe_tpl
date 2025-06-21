#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    QuotedIdent(String),
    Equal,
    Dot,
    Newline,
    Eof,
}
