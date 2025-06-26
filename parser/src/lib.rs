mod parse_impl;
mod schema_impl;
mod syntax;
mod token_analyzer;
mod validator;
mod value_impl;

use kernel::{error::ParseError, parse::Parse, schema::Schema, token::Token, value::Value};

use validator::syntax::{ConfigRule, SchemaRule, SyntaxValidator};

use token_analyzer::TokenAnalyzer;

pub struct Parser;

impl Parse<Value> for Parser {
    fn parse(tokens: Vec<Token>) -> Result<Value, ParseError> {
        let syntax = TokenAnalyzer::analyze(tokens)?;
        SyntaxValidator::validate::<ConfigRule>(&syntax)?;
        Self::convert_to(syntax)
    }
}

pub struct SchemaParser;

impl Parse<Schema> for Parser {
    fn parse(tokens: Vec<Token>) -> Result<Schema, ParseError> {
        let syntax = TokenAnalyzer::analyze(tokens)?;
        SyntaxValidator::validate::<SchemaRule>(&syntax)?;
        Self::convert_to(syntax)
    }
}
