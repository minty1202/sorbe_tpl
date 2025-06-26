use super::schema_impl::SchemaExt;
use super::value_impl::ValueExt;
use kernel::{schema::Schema, value::Value};

#[derive(Debug, PartialEq)]
pub enum SyntaxValue {
    Plain(String),
    Quoted(String),
}

impl From<SyntaxValue> for Value {
    fn from(syntax_value: SyntaxValue) -> Self {
        match syntax_value {
            SyntaxValue::Plain(s) => Value::from_plain_string(s),
            SyntaxValue::Quoted(s) => Value::String(s),
        }
    }
}

impl From<SyntaxValue> for Schema {
    fn from(syntax_value: SyntaxValue) -> Self {
        match syntax_value {
            SyntaxValue::Plain(s) => Schema::from_symbol(&s)
                .unwrap_or_else(|| unreachable!("Unsupported schema type: {}", s)),
            SyntaxValue::Quoted(_) => {
                unreachable!("Quoted values should not be converted to Schema directly")
            }
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
    use kernel::value::{Number, Value};

    #[test]
    fn test_syntax_value_from() {
        let result: Value = SyntaxValue::Plain("true".into()).into();
        assert_eq!(result, Value::Bool(true));

        let result: Value = SyntaxValue::Plain("42".into()).into();
        assert_eq!(result, Value::Number(Number::UInt(42)));

        let result: Value = SyntaxValue::Plain(".42".into()).into();
        assert_eq!(result, Value::Number(Number::Float(0.42)));

        let result: Value = SyntaxValue::Plain("-42".into()).into();
        assert_eq!(result, Value::Number(Number::Int(-42)));

        let result: Value = SyntaxValue::Quoted("quoted string".into()).into();
        assert_eq!(result, Value::String("quoted string".into()));
    }

    #[test]
    fn test_syntax_value_schema_type_from() {
        let result: Schema = SyntaxValue::Plain("bool".into()).into();
        assert_eq!(result, Schema::Bool);

        let result: Schema = SyntaxValue::Plain("integer?".into()).into();
        assert_eq!(result, Schema::Optional(Box::new(Schema::Integer)));
    }

    #[test]
    #[should_panic]
    fn test_syntax_value_schema_type_from_quoted() {
        let _result: Schema = SyntaxValue::Quoted("quoted string".into()).into();
    }

    #[test]
    #[should_panic]
    fn test_convert_base_schema_type_invalid() {
        let _result: Schema = SyntaxValue::Plain("invalid_type".into()).into();
    }
}
