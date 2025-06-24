use kernel::{error::ParseError, shared::Map, value::Value};

use super::syntax::Syntax;

use super::Parser;

impl Parser {
    pub fn convert_to_value(syntax: Syntax) -> Result<Value, ParseError> {
        let mut result = Map::new();

        for pattern in syntax.patterns {
            let nested_value = Self::build_nested_value(pattern.key_parts, pattern.value.into());
            Self::merge_into(&mut result, nested_value);
        }

        Ok(Value::Dict(result))
    }

    fn build_nested_value(key_parts: Vec<String>, value: Value) -> Value {
        let mut iter = key_parts.iter().peekable();

        let key_part = iter.next().expect("key_parts should not be empty");

        if iter.peek().is_some() {
            let remaining: Vec<String> = iter.cloned().collect();
            let inner_value = Self::build_nested_value(remaining, value);

            let mut map = Map::new();
            map.insert(key_part.clone(), inner_value);
            Value::Dict(map)
        } else {
            let mut map = Map::new();
            map.insert(key_part.clone(), value);
            Value::Dict(map)
        }
    }

    fn merge_into(target: &mut Map<String, Value>, source: Value) {
        if let Value::Dict(source_map) = source {
            for (key, value) in source_map {
                match target.get_mut(&key) {
                    Some(Value::Dict(target_inner)) => {
                        Self::merge_into(target_inner, value);
                    }
                    _ => {
                        target.insert(key, value);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{Pattern, Syntax, SyntaxValue};

    #[test]
    fn test_simple_syntax() {
        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["a".into()],
                    value: SyntaxValue::Quoted("value1".into()),
                },
                Pattern {
                    key_parts: vec!["b".into()],
                    value: SyntaxValue::Quoted("value2".into()),
                },
            ],
        };

        let result = Parser::convert_to_value(syntax).unwrap();
        let expected = Map::from([
            ("a".to_string(), Value::String("value1".to_string())),
            ("b".to_string(), Value::String("value2".to_string())),
        ]);

        assert_eq!(result, Value::Dict(expected));
    }

    #[test]
    fn test_nested_syntax() {
        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["a".into(), "b".into()],
                    value: SyntaxValue::Quoted("value1".into()),
                },
                Pattern {
                    key_parts: vec!["a".into(), "c".into()],
                    value: SyntaxValue::Quoted("value2".into()),
                },
                Pattern {
                    key_parts: vec!["d".into()],
                    value: SyntaxValue::Quoted("value3".into()),
                },
            ],
        };

        let result = Parser::convert_to_value(syntax).unwrap();
        let expected = Map::from([
            (
                "a".to_string(),
                Value::Dict(Map::from([
                    ("b".to_string(), Value::String("value1".to_string())),
                    ("c".to_string(), Value::String("value2".to_string())),
                ])),
            ),
            ("d".to_string(), Value::String("value3".to_string())),
        ]);

        assert_eq!(result, Value::Dict(expected));
    }

    #[test]
    fn test_deep_nested_syntax() {
        let syntax = Syntax {
            patterns: vec![
                Pattern {
                    key_parts: vec!["a".into(), "b".into(), "c".into()],
                    value: SyntaxValue::Quoted("value1".into()),
                },
                Pattern {
                    key_parts: vec!["a".into(), "b".into(), "d".into()],
                    value: SyntaxValue::Quoted("value2".into()),
                },
                Pattern {
                    key_parts: vec!["e".into()],
                    value: SyntaxValue::Quoted("value3".into()),
                },
            ],
        };

        let result = Parser::convert_to_value(syntax).unwrap();
        let expected = Map::from([
            (
                "a".to_string(),
                Value::Dict(Map::from([(
                    "b".to_string(),
                    Value::Dict(Map::from([
                        ("c".to_string(), Value::String("value1".to_string())),
                        ("d".to_string(), Value::String("value2".to_string())),
                    ])),
                )])),
            ),
            ("e".to_string(), Value::String("value3".to_string())),
        ]);

        assert_eq!(result, Value::Dict(expected));
    }
}
