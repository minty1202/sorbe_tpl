#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    QuotedIdent(String),
    Separator,
    Dot,
    Newline,
    Eof,
}
