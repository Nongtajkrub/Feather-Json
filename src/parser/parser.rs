use crate::parser::{token::{Token, TokenType}, lexer::{lex, lex_from_file}};
use std::{collections::HashMap, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Either<F, S> {
    First(F),
    Second(S),
}

enum JsonValue {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

struct KeyValPair {
    pair: HashMap<String, Either<JsonValue, KeyValPair>>,
}

impl KeyValPair {
    pub fn new() -> Self {
        KeyValPair {
            pair: HashMap::new(),
        }
    }

    #[inline]
    pub fn insert_normal(&mut self, key: &str, val: JsonValue) {
        self.pair.insert(key.to_string(), Either::First(val));
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    result: KeyValPair,
}

impl Parser {
    fn new(tokens: Vec<Token>, result: KeyValPair) -> Self {
        let mut parser = Parser {
            tokens,
            result,
        };
        parser.parse();
        parser
    }

    #[inline]
    pub fn from_file(path: &str) -> io::Result<Self> {
        Ok(Self::new(lex_from_file(path)?, KeyValPair::new()))
    }

    #[inline]
    pub fn from_string(data: &str) -> Self {
        Self::new(lex(data), KeyValPair::new())
    }

    fn string_to_val(val: &str) -> JsonValue {
        if val == "true" {
            JsonValue::Bool(true)
        } else if val == "false" {
            JsonValue::Bool(false)
        } else if let Ok(parsed) = val.parse::<i32>() {
            JsonValue::Int(parsed)
        } else if let Ok(parsed) = val.parse::<f32>() {
            JsonValue::Float(parsed)
        } else {
            JsonValue::String(val.to_string())
        }
    }

    fn handle_key(&mut self, i: usize) {
        if self.tokens[i + 2].token_type() == TokenType::Value {
            self.result.insert_normal(
                self.tokens[i].lexeme().as_ref().unwrap(),
                Self::string_to_val(self.tokens[i + 2].lexeme().as_ref().unwrap()));
        } else if self.tokens[i + 2].token_type() == TokenType::OpeningBrace {

        }
    }

    fn parse(&mut self) {
        for i in 0..self.tokens.len() {
            match self.tokens[i].token_type() {
                TokenType::Key => self.handle_key(i),
                _ => todo!(),
            }
        }
    }
}
