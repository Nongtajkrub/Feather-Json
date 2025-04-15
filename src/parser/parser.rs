use crate::parser::{token::{Token, TokenType}, lexer::{lex, lex_from_file}};
use std::{any::Any, collections::HashMap, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Either<F, S> {
    First(F),
    Second(S),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JsonDatatype {
    String,
    Int,
    Float,
    Bool,
}

trait AllowJsonDatatype {}
impl AllowJsonDatatype for i32  {}
impl AllowJsonDatatype for f32  {}
impl AllowJsonDatatype for bool {}
impl AllowJsonDatatype for String {}

struct Val {
    val: Box<dyn Any>,
    datatype: JsonDatatype,
}

impl Val {
    pub fn new<T>(val: T, datatype: JsonDatatype) -> Self
    where
        T: 'static + AllowJsonDatatype,
    {
        Val {
            val: Box::new(val),
            datatype,
        }
    }
}

struct KeyValPair<'a> {
    pair: HashMap<&'a str, Either<Val, KeyValPair<'a>>>,
}

impl<'a> KeyValPair<'a> {
    pub fn new() -> Self {
        KeyValPair {
            pair: HashMap::new(),
        }
    }

    pub fn add_normal<T>(&mut self, key: &'a str, val: T, datatype: JsonDatatype) 
    where
        T: 'static + AllowJsonDatatype,
    {
        self.pair.insert(key, Either::First(Val::new(val, datatype)));
    }    
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    result: KeyValPair<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>, result: KeyValPair<'a>) -> Self {
        let mut parser = Parser {
            tokens,
            result,
        };
        parser.parse();
        parser
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        Ok(Self::new(lex_from_file(path)?, KeyValPair::new()))
    }

    pub fn from_string(data: &str) -> Self {
        Self::new(lex(data), KeyValPair::new())
    }

    fn parse(&mut self) {
    }
}
