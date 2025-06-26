use sorbe_tpl::{
    Error, Map, Number, Value, from_reader, from_reader_with_schema, from_str, from_str_with_schema,
};

use std::io::Cursor;

mod valid {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config_content = r#"
            server.host = localhost
            server.port = 8080
            database.enabled = true
            database.max_connections = 100
        "#;

        let result = from_str(config_content);
        assert!(result.is_ok());
        let config: Value = result.unwrap();
        let mut expected = Map::new();
        let host = Value::String("localhost".into());
        let port = Value::Number(Number::UInt(8080));
        let enabled = Value::Bool(true);
        let max_connections = Value::Number(Number::UInt(100));

        let mut server = Map::new();
        server.insert("host".into(), host);
        server.insert("port".into(), port);

        let mut database = Map::new();
        database.insert("enabled".into(), enabled);
        database.insert("max_connections".into(), max_connections);
        expected.insert("server".into(), Value::Dict(server));
        expected.insert("database".into(), Value::Dict(database));

        assert_eq!(config, Value::Dict(expected));
    }

    #[test]
    fn test_complete_config_validation() {
        let config_content = r#"
            server.host = localhost
            server.port = 8080
            database.enabled = true
            database.max_connections = 100
        "#;

        let schema_content = r#"
            server.host: string
            server.port: integer
            database.enabled: bool
            database.max_connections: unsigned_integer
        "#;

        let result = from_str_with_schema(config_content, schema_content);
        assert!(result.is_ok());
        let config = result.unwrap();
        let mut expected_server = Map::new();
        expected_server.insert("host".into(), Value::String("localhost".into()));
        expected_server.insert("port".into(), Value::Number(Number::Int(8080)));

        let mut expected_database = Map::new();
        expected_database.insert("enabled".into(), Value::Bool(true));
        expected_database.insert("max_connections".into(), Value::Number(Number::UInt(100)));

        let mut expected = Map::new();
        expected.insert("server".into(), Value::Dict(expected_server));
        expected.insert("database".into(), Value::Dict(expected_database));

        assert_eq!(config, Value::Dict(expected));
    }

    #[test]
    fn test_valid_config_with_reader() {
        let config_content = r#"
            server.host = localhost
            server.port = 8080
            database.enabled = true
            database.max_connections = 100
        "#;
        let reader = Cursor::new(config_content);
        let result = from_reader(reader);
        assert!(result.is_ok());
        let config: Value = result.unwrap();
        let mut expected = Map::new();
        let host = Value::String("localhost".into());
        let port = Value::Number(Number::UInt(8080));
        let enabled = Value::Bool(true);
        let max_connections = Value::Number(Number::UInt(100));
        let mut server = Map::new();
        server.insert("host".into(), host);
        server.insert("port".into(), port);
        let mut database = Map::new();
        database.insert("enabled".into(), enabled);
        database.insert("max_connections".into(), max_connections);
        expected.insert("server".into(), Value::Dict(server));
        expected.insert("database".into(), Value::Dict(database));
        assert_eq!(config, Value::Dict(expected));
    }

    #[test]
    fn test_valid_config_with_reader_and_schema() {
        let config_content = r#"
            server.host = localhost
            server.port = 8080
            database.enabled = true
            database.max_connections = 100
        "#;
        let schema_content = r#"
            server.host: string
            server.port: integer
            database.enabled: bool
            database.max_connections: unsigned_integer
        "#;
        let config_reader = Cursor::new(config_content);
        let schema_reader = Cursor::new(schema_content);
        let result = from_reader_with_schema(config_reader, schema_reader);
        assert!(result.is_ok());
        let config = result.unwrap();
        let mut expected_server = Map::new();
        expected_server.insert("host".into(), Value::String("localhost".into()));
        expected_server.insert("port".into(), Value::Number(Number::Int(8080)));
        let mut expected_database = Map::new();
        expected_database.insert("enabled".into(), Value::Bool(true));
        expected_database.insert("max_connections".into(), Value::Number(Number::UInt(100)));
        let mut expected = Map::new();
        expected.insert("server".into(), Value::Dict(expected_server));
        expected.insert("database".into(), Value::Dict(expected_database));
        assert_eq!(config, Value::Dict(expected));
    }
}

mod invalid {
    use super::*;

    #[test]
    fn test_invalid_syntax() {
        let config_content = r#"
            server.host = localhost
            server.port = 8080
            database.enabled = true
            database.max_connections = 100
            invalid_syntax
        "#;

        let result: Result<Value, Error> = from_str(config_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_missing_field() {
        let config_content = r#"
            server.host = localhost
            database.enabled = true
        "#;

        let schema_content = r#"
            server.host: string
            server.port: integer
            database.enabled: bool
            database.max_connections: unsigned_integer
        "#;

        let result = from_str_with_schema(config_content, schema_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_type_mismatch() {
        let config_content = r#"
            server.host = localhost
            server.port = "not_a_number"
        "#;

        let schema_content = r#"
            server.host: string
            server.port: integer
        "#;

        let result = from_str_with_schema(config_content, schema_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_with_reader() {
        let config_content = r#"
            server.host = localhost
            server.port: "not_a_number"
        "#;
        let reader = Cursor::new(config_content);
        let result: Result<Value, Error> = from_reader(reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_config_with_reader_and_schema() {
        let config_content = r#"
            server.host = localhost
            server.port = "not_a_number"
        "#;
        let schema_content = r#"
            server.host: string
            server.port: integer
        "#;
        let config_reader = Cursor::new(config_content);
        let schema_reader = Cursor::new(schema_content);
        let result = from_reader_with_schema(config_reader, schema_reader);
        assert!(result.is_err());
    }
}
