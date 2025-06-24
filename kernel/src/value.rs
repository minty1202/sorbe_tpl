use crate::shared::Map;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Dict(Map<String, Value>),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}
