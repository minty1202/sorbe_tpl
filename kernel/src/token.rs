#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    QuotedIdent(String),
    Equal,
    Dot,
    Newline,
    Eof,
}
