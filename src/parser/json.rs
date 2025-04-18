use crate::parser::{lexer::{lex, lex_from_file}, token::{Token, TokenType}};
use std::io;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum JsonValue {
    Unknown,
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
}

pub struct Json {
    tokens: Vec<Token>,
}

impl Json {
    #[inline]
    pub fn from_file(path: &str) -> io::Result<Json> {
        Ok(Json { tokens: lex_from_file(path)?, })
    } 

    #[inline]
    pub fn from_string(data: &str) -> Json {
        Json { tokens: lex(data), }
    }

    fn lexeme_to_val(lexeme: &str) -> JsonValue {
        if lexeme == "true" {
            JsonValue::Bool(true)
        } else if lexeme == "false" {
            JsonValue::Bool(false)
        } else if let Ok(as_int) = lexeme.parse::<i32>() {
            JsonValue::Int(as_int)
        } else if let Ok(as_float) = lexeme.parse::<f32>() {
            JsonValue::Float(as_float)
        } else {
            JsonValue::String(lexeme.to_string())
        }
    }

    fn update_nested_level(buf: &mut usize, current_token: &Token) {
        if current_token.token_type() == TokenType::OpeningBrace {
            *buf += 1;
            println!("nested level inc: {}", *buf);
        } else if current_token.token_type() == TokenType::ClosingBrace {
            *buf -= 1;
            println!("nested level dec: {}", *buf);
        }
    }

    pub fn get<'a>(&self, keys: &[&'a str]) -> JsonValue {
        let mut key_found = 0usize;
        let mut nested_level = 0usize;

        for (i, token) in self.tokens.iter().enumerate() {
            // Ignore the first and last token which is '{' and '}'.
            if i != 0 && i != self.tokens.len() - 1  {
                Self::update_nested_level(&mut nested_level, token);
            }

            if token.token_type() == TokenType::Key
                && nested_level == key_found
                && token.lexeme().as_ref().unwrap() == keys[key_found] 
            {
                key_found += 1;

                if key_found == keys.len() {
                    return Self::lexeme_to_val(
                        self.tokens[i + 2].lexeme().as_ref().unwrap())
                }
            } 
        }

        JsonValue::Unknown
    }
}
