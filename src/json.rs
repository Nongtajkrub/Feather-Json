use crate::{error::{JsonError, JsonResult}, lexer::{lex, lex_from_file}, token::{Token, TokenType}};
use std::{fs, io};

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum JsonValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Array(Vec<JsonValue>),
}

impl JsonValue {
    /// Creates a `JsonValue` from a string slice by inferring the most appropriate 
    /// type. Unlike `From<&str>` or `From<String>`, this method attempts to convert
    /// the input into a more specific JSON type.
    /// 
    /// # Notes
    /// - `"true"` and `"false"` are parsed as `JsonValue::Bool`
    /// - Numeric strings are parsed as `JsonValue::Int` or `JsonValue::Float`
    /// - All other input is returned as `JsonValue::String`
    ///
    /// # Examples
    /// ```
    /// assert_eq!(JsonValue::from_string("true"), JsonValue::Bool(true));
    /// assert_eq!(JsonValue::from_string("42"), JsonValue::Int(42));
    /// assert_eq!(JsonValue::from_string("3.14"), JsonValue::Float(3.14));
    /// assert_eq!(JsonValue::from_string("hello"), JsonValue::String("hello".to_string()));
    /// ```
    pub(crate) fn parse(value: &str) -> JsonValue {
        if value == "true" {
            JsonValue::Bool(true)
        } else if value == "false" {
            JsonValue::Bool(false)
        } else if let Ok(as_int) = value.parse::<i32>() {
            JsonValue::Int(as_int)
        } else if let Ok(as_float) = value.parse::<f32>() {
            JsonValue::Float(as_float)
        } else {
            JsonValue::String(value.to_string())
        }
    }

    /// Consumes the `JsonValue` and converts it into a `String`, regardless of
    /// its original type. This method guarantees a string output for any `JsonValue`.
    /// It is implemented separately instead of using `Into<String>` to avoid 
    /// conflict with `TryInto<String>`.
    ///
    /// # Notes
    /// - `JsonValue::String` returns the inner string directly.
    /// - Other types are converted using their `to_string()` implementation.
    /// - `JsonValue::Array` is currently unimplemented.
    ///
    /// # Examples
    /// ```
    /// assert_eq!(JsonValue::Int(42).to_string_force(), "42");
    /// assert_eq!(JsonValue::Bool(true).to_string_force(), "true");
    /// assert_eq!(JsonValue::String("hello".into()).to_string_force(), "hello");
    /// ```
    pub(crate) fn to_string_force(self) -> String {
        match self {
            JsonValue::String(value) => value,
            JsonValue::Array(_) => todo!(),
            JsonValue::Int(value) => value.to_string(),
            JsonValue::Float(value) => value.to_string(),
            JsonValue::Bool(value) => value.to_string(),
        }
    }
}

impl From<&str> for JsonValue {
    fn from(value: &str) -> Self {
        JsonValue::String(format!("\"{}\"", value))
    }
}

impl From<i32> for JsonValue {
    fn from(value: i32) -> Self {
        JsonValue::Int(value)
    }
}

impl From<f32> for JsonValue {
    fn from(value: f32) -> Self {
        JsonValue::Float(value)
    }
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Bool(value)
    }
}

/* todo
impl From<Array> for JsonValue {
    fn from(value: Array) -> Self {
        todo!()
    }
}
*/

impl TryInto<String> for JsonValue {
    type Error = JsonError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            JsonValue::String(value) => Ok(value),
            _ => Err(JsonError::JsonValueIsNotString),
        }
    }
}

impl TryInto<i32> for JsonValue {
    type Error = JsonError;
    
    fn try_into(self) -> Result<i32, Self::Error> {
        match self {
            JsonValue::Int(value) => Ok(value),
            _ => Err(JsonError::JsonValueIsNotInteger),
        }
    }
}

impl TryInto<f32> for JsonValue {
    type Error = JsonError;
    
    fn try_into(self) -> Result<f32, Self::Error> {
        match self {
            JsonValue::Float(value) => Ok(value),
            _ => Err(JsonError::JsonValueIsNotFloat),
        }
    }
}

impl TryInto<bool> for JsonValue {
    type Error = JsonError;
    
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            JsonValue::Bool(value) => Ok(value),
            _ => Err(JsonError::JsonValueIsNotBool),
        }
    }
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

    #[inline]
    pub fn from_tokens(tokens: Vec<Token>) -> Json {
        Json { tokens }
    }

    fn update_nested_level(buf: &mut usize, current_token: &Token) {
        match current_token.token_type() {
            TokenType::OpeningBrace => *buf += 1,
            TokenType::ClosingBrace => *buf -= 1,
            _ => (),
        };
    }

    fn update_nested_level_include_brackets(buf: &mut usize, current_token: &Token) {
        match current_token.token_type() {
            TokenType::OpeningBrace | TokenType::LeftBracket => *buf += 1,
            TokenType::ClosingBrace | TokenType::RightBracket => *buf -= 1,
            _ => (),
        };
    }
   
    /// Find a specific key token index by using keys path.
    fn find_key_token_index<'a>(&self, keys: &[&'a str]) -> JsonResult<usize> {
        if keys.is_empty() { return Err(JsonError::NoPathProvided); }

        let mut key_found = 0;
        let mut nested_level = 0;

        for (i, token) in self.tokens.iter().enumerate() {
            // Skip first and last token (usually `{` and `}`)
            if i != 0 && i != self.tokens.len() - 1  {
                Self::update_nested_level(&mut nested_level, token);
            }

            if token.token_type() == TokenType::Key && nested_level == key_found {
                let key_lexeme = token.lexeme().as_ref().unwrap();

                // Ignore the quotes in key lexeme (\"key_lexeme\") -> (key_lexeme).
                if &key_lexeme[1..key_lexeme.len() - 1] == keys[key_found] {
                    key_found += 1;

                    if key_found == keys.len() {
                        return Ok(i);
                    }
                }
            } 
        }

        Err(JsonError::InvalidPath)
    }

    pub fn get<'a>(&self, keys: &[&'a str]) -> JsonResult<JsonValue> {
        self.tokens
            .get(self.find_key_token_index(keys)? + 2)
            .ok_or(JsonError::InvalidJson)
            .and_then(|value_token| {
                if value_token.token_type() == TokenType::OpeningBrace {
                    Err(JsonError::InvalidPath)
                } else {
                    Ok(JsonValue::parse(value_token.lexeme().as_ref().unwrap()))
                }
            })
    }

    #[inline]
    fn is_key_value_an_object(&self, key_index: usize) -> JsonResult<bool> {
        self.tokens.get(key_index + 2)
            .ok_or(JsonError::InvalidJson)
            .and_then(|token| {
                Ok(token.token_type() == TokenType::OpeningBrace)
            })
    }


    fn insert_tokens(
        &mut self, keys: &[&str], tokens: Vec<Token>, end_with_sep: bool
    ) -> JsonResult<()> {
        // Find where to insert token by using keys path.
        let insert_at = match self.find_key_token_index(keys) {
            Ok(i) if self.is_key_value_an_object(i)? => i + 3,
            Ok(_) => return Err(JsonError::InsertCantInsertIntoValue),
            Err(JsonError::NoPathProvided) => 1,
            Err(e) => return Err(e),
        };

        // Get tokens len before moving it.
        let tokens_len = tokens.len();
        self.tokens.splice(insert_at..insert_at, tokens);

        // Add a separator if needed.
        if !end_with_sep { return Ok(()); }
            
        match self.tokens.get(insert_at + tokens_len) {
            Some(token) if token.token_type() == TokenType::ClosingBrace => (),
            Some(_) => self.tokens.insert(
                insert_at + tokens_len, Token::no_lexeme(TokenType::Separator)),
            None => return Err(JsonError::InvalidJson),
        } 

        Ok(())
    }

    #[inline]
    pub fn insert_value(
        &mut self, keys: &[&str], key: &str, value: JsonValue
    ) -> JsonResult<()> {
        let value_as_string: String = value.to_string_force();

        self.insert_tokens(keys, vec![
            Token::new(&format!("\"{}\"", key), TokenType::Key),
            Token::no_lexeme(TokenType::Assigner),
            Token::new(&value_as_string, TokenType::Value)
        ], true)
    }

    #[inline]
    pub fn insert_object(&mut self, keys: &[&str], key: &str) -> JsonResult<()> {
        self.insert_tokens(keys, vec![
            Token::new(&format!("\"{}\"", key), TokenType::Key),
            Token::no_lexeme(TokenType::Assigner),
            Token::no_lexeme(TokenType::OpeningBrace),
            Token::no_lexeme(TokenType::ClosingBrace),
        ], true)
    }

    fn estimate_json_size(&self) -> usize {
        let size = self.tokens.iter().map(|token| {
            match token.token_type() {
                TokenType::OpeningBrace
                | TokenType::ClosingBrace
                | TokenType::LeftBracket
                | TokenType::RightBracket
                | TokenType::Assigner 
                | TokenType::Separator => 1,

                TokenType::Key | TokenType::Value => 
                    token.lexeme().as_ref().unwrap().len(),
            }
        }).sum::<usize>();

        size + (size / 2)
    }

    pub fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(self.estimate_json_size());

        for token in self.tokens.iter() {
            match token.token_type() {
                TokenType::OpeningBrace => buffer.push('{'),
                TokenType::ClosingBrace => buffer.push('}'),
                TokenType::LeftBracket => buffer.push('['),
                TokenType::RightBracket => buffer.push(']'),
                TokenType::Key => buffer.push_str(token.lexeme().as_ref().unwrap()),
                TokenType::Value => buffer.push_str(token.lexeme().as_ref().unwrap()),
                TokenType::Assigner => buffer.push(':'),
                TokenType::Separator => buffer.push(','),
            }
        }

        buffer
    }

    /// Handles formatting for closing brackets: `}` and `]`.
    ///
    /// # Notes
    /// - The `bracket` param must be either `}` or `]` or else will panic.
    fn format_handle_close_bracket(
        &self, buf: &mut String, i: usize, nested_level: usize, bracket: char,
    ) -> JsonResult<()> {
        assert!(bracket == '}' || bracket == ']', "Bracket param must be '}}' or ']]'.");

        buf.extend(std::iter::repeat('\t').take(nested_level));

        // Figure out whether a new line is needed base on the next token.
        match self.tokens.get(i + 1) {
            Some(token) if token.token_type() == TokenType::Separator => {
                buf.push(bracket);
            }
            Some(_) => {
                buf.push(bracket);
                buf.push('\n');
            }
            None => {
                if nested_level == 0 {
                    buf.push(bracket);
                    buf.push('\n');
                } else {
                    return Err(JsonError::InvalidJson);
                }
            }
        }

        Ok(())
    }

    fn format_handle_left_bracket(
        &self, buf: &mut String, i: usize, nested_level: usize
    ) {
        if self.tokens[i - 1].token_type() != TokenType::Assigner {
            buf.extend(std::iter::repeat('\t').take(nested_level - 1));
        }

        buf.push_str("[\n");
    }

    fn format_handle_key(&self, buf: &mut String, i: usize, nested_level: usize) {
        buf.extend(std::iter::repeat('\t').take(nested_level));
        buf.push_str(self.tokens[i].lexeme().as_ref().unwrap());
    }

    fn format_handle_value(&self, buf: &mut String, i: usize, nested_level: usize) {
        // Have to do this for values in an array.
        if !buf.is_empty() && buf.chars().last().unwrap() == '\n' {
            buf.extend(std::iter::repeat('\t').take(nested_level));
        }

        buf.push_str(self.tokens[i].lexeme().as_ref().unwrap());

        if self.tokens[i + 1].token_type() != TokenType::Separator {
            buf.push('\n');
        }
    }

    pub fn to_string_format(&self) -> JsonResult<String> {
        let mut buffer = String::with_capacity(self.estimate_json_size());
        let mut nested_level = 0;

        for (i, token) in self.tokens.iter().enumerate() {
            Self::update_nested_level_include_brackets(&mut nested_level, token);

            match token.token_type() {
                TokenType::OpeningBrace => 
                    buffer.push_str("{\n"),
                TokenType::ClosingBrace =>
                    self.format_handle_close_bracket(&mut buffer, i, nested_level, '}')?,
                TokenType::LeftBracket =>
                    self.format_handle_left_bracket(&mut buffer, i, nested_level),
                TokenType::RightBracket =>
                    self.format_handle_close_bracket(&mut buffer, i, nested_level, ']')?,
                TokenType::Key => 
                    self.format_handle_key(&mut buffer, i, nested_level),
                TokenType::Value => 
                    self.format_handle_value(&mut buffer, i, nested_level),
                TokenType::Assigner => 
                    buffer.push_str(": "),
                TokenType::Separator =>
                    buffer.push_str(",\n"),
            }
        }

        Ok(buffer)
    }

    pub fn write(&self, path: &str) -> io::Result<()> {
        fs::write(path, self.to_string())?;
        Ok(())
    }

    pub fn write_format(&self, path: &str) -> JsonResult<()> {
        fs::write(path, self.to_string_format()?)?;
        Ok(())
    }
}

pub struct JsonBuilder {
    tokens: Vec<Token>,
}

impl JsonBuilder {
    #[inline]
    pub fn new() -> Self {
        JsonBuilder { tokens: vec![Token::no_lexeme(TokenType::OpeningBrace)] }
    }

    #[inline]
    fn add_separator_if_needed(&mut self) {
        if matches!(
            self.tokens.last().map(|token| token.token_type()),
            Some(TokenType::Value | TokenType::ClosingBrace)) 
        {
            self.tokens.push(Token::no_lexeme(TokenType::Separator));
        } 
    }

    pub fn object(mut self, name: &str) -> Self {
        self.add_separator_if_needed();

        self.tokens.extend([
            Token::new(&format!("\"{}\"", name), TokenType::Key),
            Token::no_lexeme(TokenType::Assigner),
            Token::no_lexeme(TokenType::OpeningBrace)
        ]);
        self
    }

    pub fn value(mut self, key: &str, value: impl Into<JsonValue>) -> Self {
        self.add_separator_if_needed();

        self.tokens.extend([
            Token::new(&format!("\"{}\"", key), TokenType::Key),
            Token::no_lexeme(TokenType::Assigner),
            Token::new(&value.into().to_string_force(), TokenType::Value)
        ]);
        self
    }

    #[inline]
    pub fn object_end(mut self) -> Self {
        self.tokens.push(Token::no_lexeme(TokenType::ClosingBrace));
        self
    }

    #[inline]
    pub fn build(mut self) -> Json {
        self.tokens.push(Token::no_lexeme(TokenType::ClosingBrace));
        Json::from_tokens(self.tokens)
    }
}
