mod emit;
mod peekable_extend;
mod symbol;
use symbol::{BlockToken, SingleToken, SymbolChar};

use std::convert::TryFrom;

use kernel::{error::TokenError, token::Token, tokenize::Tokenize};

pub struct Lexer;

impl Tokenize for Lexer {
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, TokenError> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if SymbolChar::is_skip_char(c) {
                continue;
            }

            match SymbolChar::try_from(c) {
                Ok(symbol) => {
                    if let Some(token) = symbol.emit_token(&mut chars)? {
                        tokens.push(token);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        tokens.push(Token::Eof);

        Ok(tokens)
    }
}
