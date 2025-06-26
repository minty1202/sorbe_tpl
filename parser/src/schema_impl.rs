use kernel::schema::Schema;

pub trait SchemaExt {
    fn from_symbol(s: &str) -> Option<Self>
    where
        Self: Sized;
}

impl SchemaExt for Schema {
    fn from_symbol(s: &str) -> Option<Self> {
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
