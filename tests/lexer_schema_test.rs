use kernel::{token::Token, tokenize::Tokenize};
use lexer::{Lexer, SchemaSource};

#[test]
fn test_generate_eof() {
    let source = SchemaSource::new("".to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(result, vec![Token::Eof]);
}

#[test]
fn test_generate_newline() {
    let source = SchemaSource::new("\n".to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(result, vec![Token::Newline, Token::Eof]);
}

#[test]
fn test_standard_case() {
    let text = r#"
        key: value
    "#;
    let source = SchemaSource::new(text.to_string());
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
}

#[test]
fn test_multi_line_string() {
    let text = r#"
        key: true
        key.subkey: number
    "#;
    let source = SchemaSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("true".to_string()),
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("subkey".to_string()),
            Token::Separator,
            Token::Ident("number".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_with_dot_case() {
    let text = r#"
        key.subkey: value
    "#;
    let source = SchemaSource::new(text.to_string());
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
}

#[test]
fn test_with_comment_case() {
    let text = r#"
        key.subkey: value # This is a comment
    "#;
    let source = SchemaSource::new(text.to_string());
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
        # This is a comment
        key.subkey: value
        another.key: another_value
    "#;
    let source = SchemaSource::new(text.to_string());
    let result = Lexer::tokenize(source).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("subkey".to_string()),
            Token::Separator,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Ident("another".to_string()),
            Token::Dot,
            Token::Ident("key".to_string()),
            Token::Separator,
            Token::Ident("another_value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_with_invalid_case() {
    let text = r#"
        key = value
    "#;
    let source = SchemaSource::new(text.to_string());
    let result = Lexer::tokenize(source);
    assert!(result.is_err());

    let text = r#"
        key: "value"
    "#;
    let source = SchemaSource::new(text.to_string());
    let result = Lexer::tokenize(source);
    assert!(result.is_err());

    let text = r#"
        key: 'value'
    "#;
    let source = SchemaSource::new(text.to_string());
    let result = Lexer::tokenize(source);
    assert!(result.is_err());
}
