#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    lexeme: Option<String>,
    token_type: TokenType,
}

impl Token {
    pub fn new(lexeme: &str, token_type: TokenType) -> Self {
        Token {
            lexeme: Some(lexeme.to_string()),
            token_type
        }
    }

    pub fn no_lexeme(token_type: TokenType) -> Self {
        Token {
            lexeme: None,
            token_type,
        }
    } 

    pub fn lexeme(&self) -> &Option<String> {
        &self.lexeme
    }
    
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}
