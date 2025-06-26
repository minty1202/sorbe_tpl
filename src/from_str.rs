use kernel::{error::Error, parse::Parse, tokenize::Tokenize, value::Value};

use lexer::{ConfigSource, Lexer};

use parser::Parser;

pub fn from_str(input: &str) -> Result<Value, Error> {
    let source = ConfigSource::new(input.to_string());
    let tokens = Lexer::tokenize(source)?;
    let value: Value = Parser::parse(tokens)?;

    Ok(value)
}
