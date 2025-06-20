use kernel::{error::TokenError, token::Token};

#[derive(Debug, PartialEq)]
pub enum SymbolChar {
    Single(SingleToken),
    Block(BlockToken),
}

impl SymbolChar {
    pub fn is_skip_char(c: char) -> bool {
        let additional_skip_chars = [' ', '\t', '\r'];
        additional_skip_chars.contains(&c)
    }

    pub fn is_invalid_chars(c: char) -> bool {
        let invalid_chars = [
            '[', ']', '{', '}', ',', ':', ';', '!', '@', '$', '%', '^', '&', '*', '(', ')', '+',
        ];
        invalid_chars.contains(&c) || !c.is_ascii()
    }
}

impl TryFrom<char> for SymbolChar {
    type Error = TokenError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        if Self::is_skip_char(c) {
            return Err(TokenError::Internal(format!(
                "SymbolChar::try_from called with a skip character: '{}'",
                c
            )));
        }

        if Self::is_invalid_chars(c) {
            return Err(TokenError::InvalidChar(c));
        }

        if let Some(single_token) = SingleToken::from_symbol(c) {
            return Ok(Self::Single(single_token));
        }

        if let Some(block_token) = BlockToken::from_start_char(c) {
            return Ok(Self::Block(block_token));
        }

        Err(TokenError::InvalidChar(c))
    }
}

#[derive(Debug, PartialEq)]
pub enum SingleToken {
    Equal,
    Dot,
    Newline,
}

impl SingleToken {
    fn from_symbol(c: char) -> Option<Self> {
        match c {
            '=' => Some(Self::Equal),
            '.' => Some(Self::Dot),
            '\n' => Some(Self::Newline),
            _ => None,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Equal => '=',
            Self::Dot => '.',
            Self::Newline => '\n',
        }
    }

    pub fn to_token(&self) -> Token {
        match self {
            Self::Equal => Token::Equal,
            Self::Dot => Token::Dot,
            Self::Newline => Token::Newline,
        }
    }

    pub fn chars() -> (char, char) {
        (Self::Equal.as_char(), Self::Dot.as_char())
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockToken {
    Ident(char),
    SingleQuoteIdent,
    DoubleQuoteIdent,
    Comment,
}

impl BlockToken {
    fn from_start_char(c: char) -> Option<Self> {
        match c {
            '\'' => Some(Self::SingleQuoteIdent),
            '"' => Some(Self::DoubleQuoteIdent),
            '#' => Some(Self::Comment),
            _ if c.is_alphanumeric() || c == '_' => Some(Self::Ident(c)),
            _ => None,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Ident(c) => *c,
            Self::SingleQuoteIdent => '\'',
            Self::DoubleQuoteIdent => '"',
            Self::Comment => '#',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_char_from_char() {
        assert!(SymbolChar::try_from('a').is_ok());
        assert!(SymbolChar::try_from('=').is_ok());
        assert!(SymbolChar::try_from('.').is_ok());
        assert!(SymbolChar::try_from('\'').is_ok());
        assert!(SymbolChar::try_from('\n').is_ok());
        assert!(SymbolChar::try_from('"').is_ok());
        assert!(SymbolChar::try_from('#').is_ok());

        assert!(SymbolChar::try_from(' ').is_err());
        assert!(SymbolChar::try_from('あ').is_err());
        assert!(SymbolChar::try_from('[').is_err());
        assert!(SymbolChar::try_from(']').is_err());
    }

    #[test]
    fn test_single_token_from_symbol() {
        assert_eq!(SingleToken::from_symbol('='), Some(SingleToken::Equal));
        assert_eq!(SingleToken::from_symbol('.'), Some(SingleToken::Dot));
    }

    #[test]
    fn test_block_token_from_start_char() {
        assert_eq!(
            BlockToken::from_start_char('a'),
            Some(BlockToken::Ident('a'))
        );
        assert_eq!(
            BlockToken::from_start_char('\''),
            Some(BlockToken::SingleQuoteIdent)
        );
        assert_eq!(
            BlockToken::from_start_char('"'),
            Some(BlockToken::DoubleQuoteIdent)
        );
        assert_eq!(BlockToken::from_start_char('#'), Some(BlockToken::Comment));
        assert_eq!(BlockToken::from_start_char(' '), None);
    }
}
