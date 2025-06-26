use crate::shared::Map;

use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Schema {
    String,
    Bool,
    Integer,
    UnsignedInteger,
    Float,
    Optional(Box<Schema>),
    Dict(Map<String, Schema>),
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Schema::String => write!(f, "string"),
            Schema::Bool => write!(f, "bool"),
            Schema::Integer => write!(f, "integer"),
            Schema::UnsignedInteger => write!(f, "unsigned_integer"),
            Schema::Float => write!(f, "float"),
            Schema::Optional(inner) => write!(f, "{}?", inner),
            Schema::Dict(map) => {
                let dict_str: Vec<String> =
                    map.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", dict_str.join(", "))
            }
        }
    }
}
