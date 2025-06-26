use kernel::{
    error::Error,
    schema::Schema,
    shared::Map,
    value::{Number, Value},
};

pub fn validate(value: &Value, schema: &Schema) -> Result<(), Error> {
    if let Some(result) = validate_basic_types(value, schema) {
        return result;
    }

    match (value, schema) {
        (value, Schema::Optional(inner_schema)) => validate(value, inner_schema),

        (Value::Dict(value_map), Schema::Dict(schema_map)) => {
            validate_dict_keys(value_map, schema_map)?;

            for (key, schema) in schema_map {
                let value = &value_map[key];
                validate(value, schema)?;
            }
            Ok(())
        }

        _ => Err(Error::TypeMismatch {
            expected: schema.to_string(),
            found: value.to_string(),
        }),
    }
}

fn validate_dict_keys(
    value_map: &Map<String, Value>,
    schema_map: &Map<String, Schema>,
) -> Result<(), Error> {
    for key in value_map.keys() {
        if !schema_map.contains_key(key) {
            return Err(Error::UnknownKey { key: key.clone() });
        }
    }

    for key in schema_map.keys() {
        if !value_map.contains_key(key) {
            return Err(Error::MissingKey { key: key.clone() });
        }
    }

    Ok(())
}

fn validate_basic_types(value: &Value, schema: &Schema) -> Option<Result<(), Error>> {
    match (value, schema) {
        (Value::String(_), Schema::String) => Some(Ok(())),
        (Value::Bool(_), Schema::Bool) => Some(Ok(())),
        (Value::Number(num), Schema::Integer) => Some(validate_as_integer(num)),
        (Value::Number(num), Schema::UnsignedInteger) => Some(validate_as_unsigned_integer(num)),
        (Value::Number(_), Schema::Float) => Some(Ok(())),
        (Value::Null, Schema::Optional(_)) => Some(Ok(())),
        _ => None,
    }
}

fn validate_as_integer(num: &Number) -> Result<(), Error> {
    match num {
        Number::Int(_) => Ok(()),
        Number::UInt(_) => Ok(()),
        Number::Float(f) => {
            if f.fract() == 0.0 {
                Ok(())
            } else {
                Err(Error::TypeMismatch {
                    expected: "integer".into(),
                    found: f.to_string(),
                })
            }
        }
    }
}

fn validate_as_unsigned_integer(num: &Number) -> Result<(), Error> {
    match num {
        Number::UInt(_) => Ok(()),
        Number::Int(i) if *i >= 0 => Ok(()),
        Number::Float(f) if f.fract() == 0.0 && *f >= 0.0 => Ok(()),
        _ => Err(Error::TypeMismatch {
            expected: "unsigned integer".into(),
            found: num.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod valid {
        use super::*;

        #[test]
        fn test_validate_string() {
            let value = Value::String("Hello".into());
            let schema = Schema::String;
            assert!(validate(&value, &schema).is_ok());
        }

        #[test]
        fn test_validate_integer() {
            let schema = Schema::Integer;

            let value = Value::Number(Number::Int(42));
            assert!(validate(&value, &schema).is_ok());

            let value_negative_int = Value::Number(Number::Int(-42));
            assert!(validate(&value_negative_int, &schema).is_ok());

            let value_float = Value::Number(Number::Float(42.0));
            assert!(validate(&value_float, &schema).is_ok());

            let value_uint = Value::Number(Number::UInt(42));
            assert!(validate(&value_uint, &schema).is_ok());
        }

        #[test]
        fn test_validate_unsigned_integer() {
            let schema = Schema::UnsignedInteger;

            let value = Value::Number(Number::UInt(42));
            assert!(validate(&value, &schema).is_ok());

            let value_int = Value::Number(Number::Int(42));
            assert!(validate(&value_int, &schema).is_ok());

            let value_float = Value::Number(Number::Float(42.0));
            assert!(validate(&value_float, &schema).is_ok());
        }

        #[test]
        fn test_validate_float() {
            let schema = Schema::Float;

            let value = Value::Number(Number::Float(1.23));
            assert!(validate(&value, &schema).is_ok());

            let value_negative_float = Value::Number(Number::Float(-1.23));
            assert!(validate(&value_negative_float, &schema).is_ok());

            let value_int = Value::Number(Number::Int(1));
            assert!(validate(&value_int, &schema).is_ok());

            let value_negative_int = Value::Number(Number::Int(-1));
            assert!(validate(&value_negative_int, &schema).is_ok());

            let value_uint = Value::Number(Number::UInt(1));
            assert!(validate(&value_uint, &schema).is_ok());
        }

        #[test]
        fn test_validate_bool() {
            let schema = Schema::Bool;

            let value = Value::Bool(true);
            assert!(validate(&value, &schema).is_ok());

            let value_false = Value::Bool(false);
            assert!(validate(&value_false, &schema).is_ok());
        }

        #[test]
        fn test_validate_optional() {
            let schema = Schema::Optional(Box::new(Schema::String));

            let value_null = Value::Null;
            assert!(validate(&value_null, &schema).is_ok());

            let value_string = Value::String("hello".into());
            assert!(validate(&value_string, &schema).is_ok());
        }

        #[test]
        fn test_validate_simple_dict() {
            let mut value_map = Map::new();
            value_map.insert("key1".into(), Value::String("value1".into()));
            value_map.insert("key2".into(), Value::Number(Number::Int(10)));

            let mut schema_map = Map::new();
            schema_map.insert("key1".into(), Schema::String);
            schema_map.insert("key2".into(), Schema::Integer);

            let value = Value::Dict(value_map);
            let schema = Schema::Dict(schema_map);

            assert!(validate(&value, &schema).is_ok());
        }

        #[test]
        fn test_validate_nested_dict() {
            let mut inner_value_map = Map::new();
            inner_value_map.insert("name".into(), Value::String("John".into()));
            inner_value_map.insert("age".into(), Value::Number(Number::UInt(25)));

            let mut outer_value_map = Map::new();
            outer_value_map.insert("user".into(), Value::Dict(inner_value_map));
            outer_value_map.insert("active".into(), Value::Bool(true));

            let mut inner_schema_map = Map::new();
            inner_schema_map.insert("name".into(), Schema::String);
            inner_schema_map.insert("age".into(), Schema::UnsignedInteger);

            let mut outer_schema_map = Map::new();
            outer_schema_map.insert("user".into(), Schema::Dict(inner_schema_map));
            outer_schema_map.insert("active".into(), Schema::Bool);

            let value = Value::Dict(outer_value_map);
            let schema = Schema::Dict(outer_schema_map);

            assert!(validate(&value, &schema).is_ok());
        }
    }

    mod invalid {
        use super::*;

        #[test]
        fn test_validate_string_with_integer() {
            let schema = Schema::String;
            let value = Value::Number(Number::Int(42));

            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_integer_with_string() {
            let schema = Schema::Integer;
            let value = Value::String("Hello".into());

            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_number_with_float() {
            let schema = Schema::Integer;
            let value = Value::Number(Number::Float(1.23));
            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_unsigned_integer_with_negative_integer() {
            let schema = Schema::UnsignedInteger;
            let value = Value::Number(Number::Int(-42));
            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_unsigned_integer_with_negative_float() {
            let schema = Schema::UnsignedInteger;
            let value = Value::Number(Number::Float(-1.5));
            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_bool_with_string() {
            let schema = Schema::Bool;
            let value = Value::String("true".into());
            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_optional_type_mismatch() {
            let schema = Schema::Optional(Box::new(Schema::Integer));
            let value = Value::String("hello".into()); // 文字列だが、Optional<Integer>
            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_missing_key() {
            let mut value_map = Map::new();
            value_map.insert("key1".into(), Value::String("value1".into()));

            let mut schema_map = Map::new();
            schema_map.insert("key1".into(), Schema::String);
            schema_map.insert("key2".into(), Schema::Integer);

            let value = Value::Dict(value_map);
            let schema = Schema::Dict(schema_map);

            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_unknown_key() {
            let mut value_map = Map::new();
            value_map.insert("key1".into(), Value::String("value1".into()));
            value_map.insert("key2".into(), Value::Number(Number::Int(10)));

            let mut schema_map = Map::new();
            schema_map.insert("key1".into(), Schema::String);

            let value = Value::Dict(value_map);
            let schema = Schema::Dict(schema_map);

            assert!(validate(&value, &schema).is_err());
        }

        #[test]
        fn test_validate_nested_dict_type_mismatch() {
            let mut inner_value_map = Map::new();
            inner_value_map.insert("name".into(), Value::Number(Number::Int(123))); // 型不一致

            let mut outer_value_map = Map::new();
            outer_value_map.insert("user".into(), Value::Dict(inner_value_map));

            let mut inner_schema_map = Map::new();
            inner_schema_map.insert("name".into(), Schema::String); // String期待

            let mut outer_schema_map = Map::new();
            outer_schema_map.insert("user".into(), Schema::Dict(inner_schema_map));

            let value = Value::Dict(outer_value_map);
            let schema = Schema::Dict(outer_schema_map);

            assert!(validate(&value, &schema).is_err());
        }
    }
}
