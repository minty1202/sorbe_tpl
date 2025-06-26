use serde::Deserialize;
use sorbe_tpl::{Error, Number, Value, from_reader, from_str};

#[derive(Deserialize, Debug, PartialEq)]
struct SimpleConfig {
    name: String,
    port: u16,
    enabled: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct NestedConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Deserialize, Debug, PartialEq)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

#[test]
fn test_simple_struct_deserialize() {
    let config_str = r#"
        name = "test_server"
        port = 8080
        enabled = true
    "#;

    let config: SimpleConfig = from_str(config_str).unwrap();

    assert_eq!(config.name, "test_server");
    assert_eq!(config.port, 8080);
    assert!(config.enabled);
}

#[test]
fn test_nested_struct_deserialize() {
    let config_str = r#"
        server.host = "localhost"
        server.port = 8080
        database.url = "postgres://localhost/mydb"
        database.max_connections = 100
    "#;

    let config: NestedConfig = from_str(config_str).unwrap();

    assert_eq!(config.server.host, "localhost");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.database.url, "postgres://localhost/mydb");
    assert_eq!(config.database.max_connections, 100);

    assert_eq!(config.server.host, "localhost");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.database.url, "postgres://localhost/mydb");
    assert_eq!(config.database.max_connections, 100);
}

#[test]
fn test_value_deserialize() {
    let config_str = r#"
        name = "test"
        number = 42
        flag = true
    "#;

    let value: Value = from_str(config_str).unwrap();

    match value {
        Value::Dict(dict) => {
            assert_eq!(dict.len(), 3);
            assert!(dict.contains_key("name"));
            assert!(dict.contains_key("number"));
            assert!(dict.contains_key("flag"));

            assert_eq!(dict["name"], Value::String("test".to_string()));
            assert_eq!(dict["number"], Value::Number(Number::UInt(42)));
            assert_eq!(dict["flag"], Value::Bool(true));
        }
        _ => panic!("Expected Dict"),
    }
}

#[test]
fn test_optional_fields() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct ConfigWithOptional {
        name: String,
        port: Option<u16>,
        debug: Option<bool>,
    }

    let config_str = r#"
        name = "test"
        port = 8080
    "#;

    let config: ConfigWithOptional = from_str(config_str).unwrap();

    assert_eq!(config.name, "test");
    assert_eq!(config.port, Some(8080));
    assert_eq!(config.debug, None);
}

#[test]
fn test_number_types() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct NumberConfig {
        int_val: i64,
        uint_val: u64,
        float_val: f64,
        small_int: i32,
        small_uint: u32,
        tiny_int: i8,
        tiny_uint: u8,
        medium_int: i16,
        medium_uint: u16,
        float32_val: f32,
    }

    let config_str = r#"
        int_val = -123
        uint_val = 456
        float_val = 1.23
        small_int = 42
        small_uint = 24
        tiny_int = -5
        tiny_uint = 200
        medium_int = -1000
        medium_uint = 50000
        float32_val = 1.2345
    "#;

    let config: NumberConfig = from_str(config_str).unwrap();

    assert_eq!(config.int_val, -123);
    assert_eq!(config.uint_val, 456);
    assert_eq!(config.float_val, 1.23);
    assert_eq!(config.small_int, 42);
    assert_eq!(config.small_uint, 24);
    assert_eq!(config.tiny_int, -5);
    assert_eq!(config.tiny_uint, 200);
    assert_eq!(config.medium_int, -1000);
    assert_eq!(config.medium_uint, 50000);
    assert_eq!(config.float32_val, 1.2345);

    assert_eq!(std::any::type_name_of_val(&config.int_val), "i64");
    assert_eq!(std::any::type_name_of_val(&config.uint_val), "u64");
    assert_eq!(std::any::type_name_of_val(&config.float_val), "f64");
    assert_eq!(std::any::type_name_of_val(&config.small_int), "i32");
    assert_eq!(std::any::type_name_of_val(&config.small_uint), "u32");
    assert_eq!(std::any::type_name_of_val(&config.tiny_int), "i8");
    assert_eq!(std::any::type_name_of_val(&config.tiny_uint), "u8");
    assert_eq!(std::any::type_name_of_val(&config.medium_int), "i16");
    assert_eq!(std::any::type_name_of_val(&config.medium_uint), "u16");
    assert_eq!(std::any::type_name_of_val(&config.float32_val), "f32");
}

#[test]
fn test_extra_fields_ignored() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct SimpleConfig {
        name: String,
        value: u32,
    }

    let config_str = r#"
        name = "test"
        value = 42
        extra_field = "unexpected"
        another_extra = 123
    "#;

    let result: Result<SimpleConfig, Error> = from_str(config_str);
    assert!(result.is_ok(), "Extra fields should be ignored by default");

    let config = result.unwrap();
    assert_eq!(config.name, "test");
    assert_eq!(config.value, 42);
}

#[test]
fn test_error_handling() {
    #[derive(Deserialize, Debug)]
    struct StrictConfig {
        _required_field: String,
    }

    let config_str = r#"
        wrong_field = "value"
    "#;

    let result: Result<StrictConfig, Error> = from_str(config_str);
    assert!(result.is_err());
}

#[test]
fn test_from_reader() {
    use std::io::Cursor;

    let config_data = r#"
        name = "reader_test"
        value = 42
        port = 8080
        enabled = true
    "#;

    let cursor = Cursor::new(config_data);
    let config: SimpleConfig = from_reader(cursor).unwrap();

    assert_eq!(config.name, "reader_test");
}

#[test]
fn test_string_values() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct StringConfig {
        simple: String,
        quoted: String,
        empty: String,
    }

    let config_str = r#"
        simple = hello
        quoted = "hello world"
        empty = ""
    "#;

    let config: StringConfig = from_str(config_str).unwrap();

    assert_eq!(config.simple, "hello");
    assert_eq!(config.quoted, "hello world");
    assert_eq!(config.empty, "");
}

#[test]
fn test_type_mismatch_errors() {
    #[derive(Deserialize, Debug)]
    struct TypedConfig {
        _number: u32,
        _flag: bool,
    }

    let bad_config = r#"
        number = "not_a_number"
        flag = true
    "#;

    let result: Result<TypedConfig, Error> = from_str(bad_config);
    assert!(result.is_err());

    let bad_config2 = r#"
        number = 42
        flag = 123
    "#;

    let result2: Result<TypedConfig, Error> = from_str(bad_config2);
    assert!(result2.is_err());
}

#[test]
fn test_number_overflow() {
    #[derive(Deserialize, Debug)]
    struct SmallNumbers {
        _tiny: u8,
    }

    // u8の範囲を超える値
    let config_str = r#"
        tiny = 300
    "#;

    let result: Result<SmallNumbers, Error> = from_str(config_str);
    assert!(result.is_err(), "Should fail for overflow");
}

#[test]
fn test_float_special_values() {
    #[derive(Deserialize, Debug)]
    struct FloatConfig {
        zero: f64,
        very_small: f64,
    }

    let config_str = r#"
        zero = 0.0
        very_small = 0.000001
    "#;

    let config: FloatConfig = from_str(config_str).unwrap();
    assert_eq!(config.zero, 0.0);
    assert_eq!(config.very_small, 0.000001);
}

#[test]
fn test_deep_nesting() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct DeepConfig {
        level1: Level1,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Level1 {
        level2: Level2,
        value: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Level2 {
        level3: Level3,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Level3 {
        deep_value: i32,
    }

    let config_str = r#"
        level1.value = "test"
        level1.level2.level3.deep_value = 42
    "#;

    let config: DeepConfig = from_str(config_str).unwrap();
    assert_eq!(config.level1.value, "test");
    assert_eq!(config.level1.level2.level3.deep_value, 42);
}
