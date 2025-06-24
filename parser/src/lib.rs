mod parse_impl;
mod syntax;
mod validator;

use kernel::{error::ParseError, parse::Parse, token::Token, value::Value};

use validator::{syntax::SyntaxValidator, token::TokenValidator};

pub struct Parser;

impl Parse for Parser {
    fn parse(&self, tokens: Vec<Token>) -> Result<Value, ParseError> {
        let syntax = TokenValidator::validate(tokens)?;
        SyntaxValidator::validate(&syntax)?;
        Parser::convert_to_value(syntax)
    }
}
