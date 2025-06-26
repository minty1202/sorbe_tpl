use kernel::value::{Number, Value};

pub trait ValueExt {
    fn from_plain_string(s: String) -> Self;
}

impl ValueExt for Value {
    fn from_plain_string(s: String) -> Self {
        if s == "true" || s == "false" {
            return Value::Bool(s.parse().unwrap());
        }

        if s.contains('.') {
            if let Ok(float) = s.parse::<f64>() {
                return Value::Number(Number::Float(float));
            }
        }

        if let Ok(uint) = s.parse::<u64>() {
            Value::Number(Number::UInt(uint))
        } else if let Ok(int) = s.parse::<i64>() {
            Value::Number(Number::Int(int))
        } else {
            Value::String(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_plain_string() {
        assert_eq!(
            Value::from_plain_string("true".to_string()),
            Value::Bool(true)
        );
        assert_eq!(
            Value::from_plain_string("false".to_string()),
            Value::Bool(false)
        );
        assert_eq!(
            Value::from_plain_string("123".to_string()),
            Value::Number(Number::UInt(123))
        );
        assert_eq!(
            Value::from_plain_string("-123".to_string()),
            Value::Number(Number::Int(-123))
        );
        assert_eq!(
            Value::from_plain_string("123.45".to_string()),
            Value::Number(Number::Float(123.45))
        );
        assert_eq!(
            Value::from_plain_string("0".to_string()),
            Value::Number(Number::UInt(0))
        );
        assert_eq!(
            Value::from_plain_string(".42".to_string()),
            Value::Number(Number::Float(0.42))
        );
        assert_eq!(
            Value::from_plain_string("-1.234".to_string()),
            Value::Number(Number::Float(-1.234))
        );
        assert_eq!(
            Value::from_plain_string("hello".to_string()),
            Value::String("hello".to_string())
        );
        assert_eq!(
            Value::from_plain_string("".to_string()),
            Value::String("".to_string())
        );
    }
}
