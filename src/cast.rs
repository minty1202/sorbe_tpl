use kernel::{
    schema::Schema,
    shared::Map,
    value::{Number, Value},
};

pub fn cast(value: &Value, schema: &Schema) -> Value {
    match (value, schema) {
        (Value::Number(num), Schema::Integer) => cast_as_integer(num),
        (Value::Number(num), Schema::UnsignedInteger) => cast_as_unsigned_integer(num),

        (value, Schema::Optional(inner_schema)) => cast(value, inner_schema),

        (Value::Dict(value_map), Schema::Dict(schema_map)) => {
            let mut casted_map = Map::new();
            for (key, inner_schema) in schema_map {
                let inner_value = &value_map[key];
                casted_map.insert(key.clone(), cast(inner_value, inner_schema));
            }
            Value::Dict(casted_map)
        }

        _ => value.clone(),
    }
}

fn cast_as_integer(num: &Number) -> Value {
    match num {
        Number::Int(i) => Value::Number(Number::Int(*i)),
        Number::UInt(u) => Value::Number(Number::Int(*u as i64)),
        Number::Float(f) => Value::Number(Number::Int(*f as i64)),
    }
}

fn cast_as_unsigned_integer(num: &Number) -> Value {
    match num {
        Number::Int(i) => Value::Number(Number::UInt(*i as u64)),
        Number::UInt(u) => Value::Number(Number::UInt(*u)),
        Number::Float(f) => Value::Number(Number::UInt(*f as u64)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_as_integer() {
        assert_eq!(
            cast_as_integer(&Number::Int(42)),
            Value::Number(Number::Int(42))
        );
        assert_eq!(
            cast_as_integer(&Number::UInt(42)),
            Value::Number(Number::Int(42))
        );
        assert_eq!(
            cast_as_integer(&Number::Float(42.0)),
            Value::Number(Number::Int(42))
        );
    }

    #[test]
    fn test_cast_as_unsigned_integer() {
        assert_eq!(
            cast_as_unsigned_integer(&Number::Int(42)),
            Value::Number(Number::UInt(42))
        );
        assert_eq!(
            cast_as_unsigned_integer(&Number::UInt(42)),
            Value::Number(Number::UInt(42))
        );
        assert_eq!(
            cast_as_unsigned_integer(&Number::Float(42.0)),
            Value::Number(Number::UInt(42))
        );
    }

    #[test]
    fn test_cast_with_schema() {
        let value = Value::Number(Number::UInt(42));
        let schema = Schema::Integer;

        assert_eq!(cast(&value, &schema), Value::Number(Number::Int(42)));
    }

    #[test]
    fn test_cast_optional() {
        let value = Value::Number(Number::UInt(42));
        let schema = Schema::Optional(Box::new(Schema::Integer));

        assert_eq!(cast(&value, &schema), Value::Number(Number::Int(42)));
    }

    #[test]
    fn test_cast_no_change() {
        let value = Value::String("hello".into());
        let schema = Schema::String;

        assert_eq!(cast(&value, &schema), value);
    }

    #[test]
    fn test_cast_dict() {
        let mut value_map = Map::new();
        value_map.insert("port".into(), Value::Number(Number::UInt(8080)));

        let mut schema_map = Map::new();
        schema_map.insert("port".into(), Schema::Integer);

        let value = Value::Dict(value_map);
        let schema = Schema::Dict(schema_map);

        let result = cast(&value, &schema);

        let mut expected_map = Map::new();
        expected_map.insert("port".into(), Value::Number(Number::Int(8080)));
        assert_eq!(result, Value::Dict(expected_map));
    }
}
