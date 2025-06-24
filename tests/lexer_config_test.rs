use kernel::{token::Token, tokenize::Tokenize};
use lexer::{ConfigSource, Lexer};

#[test]
fn test_generate_eof() {
    let source = ConfigSource::new("".to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(result, vec![Token::Eof]);
}

#[test]
fn test_generate_newline() {
    let source = ConfigSource::new("\n".to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(result, vec![Token::Newline, Token::Eof]);
}

#[test]
fn test_standard_case() {
    let text = r#"
        key = value
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = 'value'
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = "value"
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_multi_line_string() {
    let text = r#"
        key1 = value1
        key2 = value2
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Separator,
            Token::Ident("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Separator,
            Token::Ident("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key1 = 'value1'
        key2 = 'value2'
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Separator,
            Token::QuotedIdent("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Separator,
            Token::QuotedIdent("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key1 = "value1"
        key2 = "value2"
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Separator,
            Token::QuotedIdent("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Separator,
            Token::QuotedIdent("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_non_value_case() {
    let text = r#"
        key =
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key1 = value1
        key2 = 
        key3 = value3
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Separator,
            Token::Ident("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Separator,
            Token::Newline,
            Token::Ident("key3".to_string()),
            Token::Separator,
            Token::Ident("value3".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_with_dot_case() {
    let text = r#"
        key.subkey = value
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("subkey".to_string()),
            Token::Separator,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key.subkey = 'value'
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("subkey".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key.subkey = "value"
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("subkey".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_with_comment_case() {
    let text = r#"
        key = value # comment
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        # comment
        key = value
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = 'value' # comment
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        # comment
        key = 'value'
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = "value" # comment
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        # comment
        key = "value"
    "#;
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_invalid_case() {
    let text = "key = 'value";
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unterminated string literal"
    );

    let text = "key = \"value";
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unterminated string literal"
    );

    let text = "@key = value";
    let source = ConfigSource::new(text.to_string());
    let result = Lexer::tokenize(source);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid character: '@'");
}
