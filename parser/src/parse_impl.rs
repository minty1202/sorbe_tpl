use kernel::{error::ParseError, schema::Schema, shared::Map, value::Value};

use super::syntax::{Syntax, SyntaxValue};

use super::Parser;

pub trait DictLike {
    fn from_map(map: Map<String, Self>) -> Self
    where
        Self: Sized;
    fn to_map_mut(&mut self) -> Option<&mut Map<String, Self>>
    where
        Self: Sized;
}

impl DictLike for Value {
    fn from_map(map: Map<String, Self>) -> Self {
        Value::Dict(map)
    }

    fn to_map_mut(&mut self) -> Option<&mut Map<String, Self>> {
        match self {
            Value::Dict(map) => Some(map),
            _ => None,
        }
    }
}

impl DictLike for Schema {
    fn from_map(map: Map<String, Self>) -> Self {
        Schema::Dict(map)
    }

    fn to_map_mut(&mut self) -> Option<&mut Map<String, Self>> {
        match self {
            Schema::Dict(map) => Some(map),
            _ => None,
        }
    }
}

impl Parser {
    pub fn convert_to<T>(syntax: Syntax) -> Result<T, ParseError>
    where
        T: DictLike + From<SyntaxValue>,
    {
        let mut result = Map::new();

        for pattern in syntax.patterns {
            let value: T = pattern.value.into();
            Self::insert_nested_path(&mut result, &pattern.key_parts, value);
        }

        Ok(T::from_map(result))
    }

    fn insert_nested_path<T>(target: &mut Map<String, T>, path: &[String], value: T)
    where
        T: DictLike,
    {
        let (first, rest) = path
            .split_first()
            .unwrap_or_else(|| unreachable!("path should not be empty"));

        if rest.is_empty() {
            target.insert(first.clone(), value);
            return;
        }

        let nested = target
            .entry(first.clone())
            .or_insert_with(|| T::from_map(Map::new()));

        let Some(nested_map) = nested.to_map_mut() else {
            unreachable!("Should always be Dict")
        };

        Self::insert_nested_path(nested_map, rest, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{Pattern, Syntax, SyntaxValue};

    mod value {
        use super::*;

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

            let result: Value = Parser::convert_to(syntax).unwrap();
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

            let result: Value = Parser::convert_to(syntax).unwrap();
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

            let result: Value = Parser::convert_to(syntax).unwrap();
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

    mod schema {
        use super::*;

        #[test]
        fn test_simple_syntax() {
            let syntax = Syntax {
                patterns: vec![
                    Pattern {
                        key_parts: vec!["a".into()],
                        value: SyntaxValue::Plain("string".into()),
                    },
                    Pattern {
                        key_parts: vec!["b".into()],
                        value: SyntaxValue::Plain("string".into()),
                    },
                ],
            };

            let result: Schema = Parser::convert_to(syntax).unwrap();
            let expected = Map::from([
                ("a".to_string(), Schema::String),
                ("b".to_string(), Schema::String),
            ]);

            assert_eq!(result, Schema::Dict(expected));
        }

        #[test]
        fn test_nested_syntax() {
            let syntax = Syntax {
                patterns: vec![
                    Pattern {
                        key_parts: vec!["a".into(), "b".into()],
                        value: SyntaxValue::Plain("string".into()),
                    },
                    Pattern {
                        key_parts: vec!["a".into(), "c".into()],
                        value: SyntaxValue::Plain("string".into()),
                    },
                    Pattern {
                        key_parts: vec!["d".into()],
                        value: SyntaxValue::Plain("string".into()),
                    },
                ],
            };

            let result: Schema = Parser::convert_to(syntax).unwrap();
            let expected = Map::from([
                (
                    "a".to_string(),
                    Schema::Dict(Map::from([
                        ("b".to_string(), Schema::String),
                        ("c".to_string(), Schema::String),
                    ])),
                ),
                ("d".to_string(), Schema::String),
            ]);

            assert_eq!(result, Schema::Dict(expected));
        }
    }
}
