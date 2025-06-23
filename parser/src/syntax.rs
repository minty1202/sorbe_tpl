#[derive(Debug, PartialEq)]
pub enum SyntaxValue {
    Plain(String),
    Quoted(String),
}

#[derive(Debug, PartialEq)]
pub struct Syntax {
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub key_parts: Vec<String>,
    pub value: SyntaxValue,
}
