use kernel::{error::Error, parse::Parse, tokenize::Tokenize, value::Value};
use lexer::{ConfigSource, Lexer};
use parser::Parser;
use serde::de::DeserializeOwned;

fn parse_config(input: &str) -> Result<Value, Error> {
    let source = ConfigSource::new(input.to_string());
    let tokens = Lexer::tokenize(source)?;
    let value: Value = Parser::parse(tokens)?;
    Ok(value)
}

pub fn from_str<T>(input: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let value = parse_config(input)?;
    T::deserialize(value).map_err(|e| Error::Serde(e.to_string()))
}
