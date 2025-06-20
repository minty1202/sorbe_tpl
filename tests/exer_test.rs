use kernel::{token::Token, tokenize::Tokenize};
use lexer::Lexer;

#[test]
fn test_error_on_invalid_char() {
    let lexer = Lexer;
    let result = lexer.tokenize("a$").unwrap_err();
    assert_eq!(result.to_string(), "Invalid character: '$'");

    let result = lexer.tokenize("a!").unwrap_err();
    assert_eq!(result.to_string(), "Invalid character: '!'");
}

#[test]
fn test_generate_eof() {
    let lexer = Lexer;
    let result = lexer.tokenize("").unwrap();
    assert_eq!(result, vec![Token::Eof]);
}

#[test]
fn test_generate_newline() {
    let lexer = Lexer;
    let result = lexer.tokenize("\n").unwrap();
    assert_eq!(result, vec![Token::Newline, Token::Eof]);
}

#[test]
fn test_generate_single_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("a").unwrap();
    assert_eq!(result, vec![Token::Ident('a'.to_string()), Token::Eof]);
}

#[test]
fn test_error_on_single_jp_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("あ").unwrap_err();
    assert_eq!(result.to_string(), "Invalid character: 'あ'");
}

#[test]
fn test_generate_multi_char_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("ab").unwrap();
    assert_eq!(result, vec![Token::Ident("ab".to_string()), Token::Eof]);
}

#[test]
fn test_generate_single_quote_single_char_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("'a'").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("a".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_single_quote_jp_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("'あ'").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("あ".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_single_quote_multi_char_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("'ab'").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("ab".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_single_quote_included_line_break() {
    let lexer = Lexer;
    let result = lexer.tokenize(r"'a\nb'").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("a\\nb".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_double_quote_single_char_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("\"a\"").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("a".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_double_quote_jp_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("\"あ\"").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("あ".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_double_quote_multi_char_identifier() {
    let lexer = Lexer;
    let result = lexer.tokenize("\"ab\"").unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("ab".to_string()), Token::Eof]
    );
}

#[test]
fn test_generate_double_quote_included_line_break() {
    let lexer = Lexer;
    let result = lexer.tokenize(r#""a\nb""#).unwrap();
    assert_eq!(
        result,
        vec![Token::QuotedIdent("a\nb".to_string()), Token::Eof]
    );
}

#[test]
fn test_skip_comment() {
    let lexer = Lexer;
    let result = lexer.tokenize("# comment\n").unwrap();
    assert_eq!(result, vec![Token::Newline, Token::Eof]);

    let result = lexer.tokenize("# comment\n# another comment\n").unwrap();
    assert_eq!(result, vec![Token::Newline, Token::Newline, Token::Eof]);

    let result = lexer.tokenize("test # comment").unwrap();
    assert_eq!(result, vec![Token::Ident("test".to_string()), Token::Eof]);

    let text = r#"
        # comment
        test
        another
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("test".to_string()),
            Token::Newline,
            Token::Ident("another".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_standard_case() {
    let lexer = Lexer;
    let text = r#"
        key = value
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = 'value'
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = "value"
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_multi_line_string() {
    let lexer = Lexer;
    let text = r#"
        key1 = value1
        key2 = value2
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Equal,
            Token::Ident("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Equal,
            Token::Ident("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key1 = 'value1'
        key2 = 'value2'
    "#;
    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Equal,
            Token::QuotedIdent("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Equal,
            Token::QuotedIdent("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key1 = "value1"
        key2 = "value2"
    "#;
    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key1".to_string()),
            Token::Equal,
            Token::QuotedIdent("value1".to_string()),
            Token::Newline,
            Token::Ident("key2".to_string()),
            Token::Equal,
            Token::QuotedIdent("value2".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_with_dot_case() {
    let lexer = Lexer;
    let text = r#"
        key. value = value
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("value".to_string()),
            Token::Equal,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key.value = 'value'
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("value".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key.value = "value"
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Dot,
            Token::Ident("value".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_with_comment_case() {
    let lexer = Lexer;
    let text = r#"
        key = value # comment
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let lexer = Lexer;
    let text = r#"
        # comment
        key = value
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::Ident("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = 'value' # comment
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        # comment
        key = 'value'
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        key = "value" # comment
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );

    let text = r#"
        # comment
        key = "value"
    "#;

    let result = lexer.tokenize(text).unwrap();
    assert_eq!(
        result,
        vec![
            Token::Newline,
            Token::Newline,
            Token::Ident("key".to_string()),
            Token::Equal,
            Token::QuotedIdent("value".to_string()),
            Token::Newline,
            Token::Eof,
        ]
    );
}

#[test]
fn test_invalid_case() {
    let lexer = Lexer;
    let text = "key = 'value";

    let result = lexer.tokenize(text);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unterminated string literal"
    );

    let text = "key = \"value";

    let result = lexer.tokenize(text);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Unterminated string literal"
    );

    let text = "@key = value";
    let result = lexer.tokenize(text);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid character: '@'");
}
