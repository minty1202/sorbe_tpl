use kernel::value::{Number, Value};

#[derive(Debug, PartialEq)]
pub enum SyntaxValue {
    Plain(String),
    Quoted(String),
}

fn convert_plain(s: String) -> Value {
    if s == "true" || s == "false" {
        return Value::Bool(s.parse().unwrap());
    }

    if let Ok(float) = s.parse::<f64>() {
        if s.contains('.') {
            Value::Number(Number::Float(float))
        } else {
            Value::Number(Number::Int(float as i64))
        }
    } else {
        Value::String(s)
    }
}

impl From<SyntaxValue> for Value {
    fn from(syntax_value: SyntaxValue) -> Self {
        match syntax_value {
            SyntaxValue::Plain(s) => convert_plain(s),
            SyntaxValue::Quoted(s) => Value::String(s),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_plain() {
        assert_eq!(convert_plain("true".into()), Value::Bool(true));
        assert_eq!(convert_plain("false".into()), Value::Bool(false));
        assert_eq!(convert_plain("42".into()), Value::Number(Number::Int(42)));
        assert_eq!(
            convert_plain("1.2".into()),
            Value::Number(Number::Float(1.2))
        );
        assert_eq!(
            convert_plain(".1".into()),
            Value::Number(Number::Float(0.1))
        );
        assert_eq!(
            convert_plain("1.0".into()),
            Value::Number(Number::Float(1.0))
        );
        assert_eq!(
            convert_plain("0.0".into()),
            Value::Number(Number::Float(0.0))
        );
        assert_eq!(convert_plain("hello".into()), Value::String("hello".into()));
    }

    #[test]
    fn test_syntax_value_from() {
        let result: Value = SyntaxValue::Plain("true".into()).into();
        assert_eq!(result, Value::Bool(true));

        let result: Value = SyntaxValue::Plain("42".into()).into();
        assert_eq!(result, Value::Number(Number::Int(42)));

        let result: Value = SyntaxValue::Quoted("quoted string".into()).into();
        assert_eq!(result, Value::String("quoted string".into()));
    }
}
