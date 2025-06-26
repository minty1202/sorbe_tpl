use crate::shared::Map;

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Dict(Map<String, Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Dict(d) => {
                let dict_str: Vec<String> =
                    d.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", dict_str.join(", "))
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i64),
    UInt(u64),
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{}", i),
            Number::UInt(u) => write!(f, "{}", u),
            Number::Float(fl) => write!(f, "{}", fl),
        }
    }
}
