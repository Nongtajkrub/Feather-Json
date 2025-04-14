#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    /// `{`
    OpeningBrace,
    /// `}`
    ClosingBrace,
    /// `key`: value
    Key,
    /// key`:` value
    Assigner,
    /// key: `value`
    Value,
    /// key: value`,`
    Separator,
}

pub struct Token {
    lexeme: String,
    token_type: TokenType,
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType) -> Self {
        Token {
            lexeme: String::from(lexeme),
            token_type
        }
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
    
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}
