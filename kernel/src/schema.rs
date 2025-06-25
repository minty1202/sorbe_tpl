use crate::shared::Map;

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

impl Schema {
    pub fn as_symbol(&self) -> String {
        match self {
            Schema::String => "string".to_string(),
            Schema::Bool => "bool".to_string(),
            Schema::Integer => "integer".to_string(),
            Schema::UnsignedInteger => "unsigned_integer".to_string(),
            Schema::Float => "float".to_string(),
            Schema::Optional(inner) => format!("{}?", inner.as_symbol()),
            Schema::Dict(_) => {
                panic!("Dict schema cannot be converted to symbol")
            }
        }
    }

    pub fn from_symbol(s: &str) -> Option<Self> {
        if let Some(base_type) = s.strip_suffix('?') {
            Self::from_symbol(base_type).map(|inner| Schema::Optional(Box::new(inner)))
        } else {
            match s {
                "bool" => Some(Schema::Bool),
                "integer" => Some(Schema::Integer),
                "unsigned_integer" => Some(Schema::UnsignedInteger),
                "float" => Some(Schema::Float),
                "string" => Some(Schema::String),
                _ => None,
            }
        }
    }
}
