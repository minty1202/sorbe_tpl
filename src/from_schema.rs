use kernel::{error::Error, parse::Parse, schema::Schema, tokenize::Tokenize};

use lexer::{Lexer, SchemaSource};

use parser::Parser;

pub fn from_schema(input: &str) -> Result<Schema, Error> {
    let source = SchemaSource::new(input.to_string());
    let tokens = Lexer::tokenize(source)?;
    let value: Schema = Parser::parse(tokens)?;
    Ok(value)
}
