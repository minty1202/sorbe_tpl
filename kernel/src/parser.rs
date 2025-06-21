use crate::shared::Map;

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Dict(Map<String, Value>),
}

#[derive(Debug)]
pub enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}
