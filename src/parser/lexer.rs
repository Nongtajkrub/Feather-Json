use crate::parser::token::{Token, TokenType};
use std::{fs, io};

fn handle_closing_brace(buf: &mut Vec<Token>, lexeme: &mut String) {
    buf.push(Token::new(lexeme, TokenType::Value));
    buf.push(Token::no_lexeme(TokenType::ClosingBrace));

    lexeme.clear();
}

fn handle_assigner(buf: &mut Vec<Token>, lexeme: &mut String) {
    buf.push(Token::new(lexeme, TokenType::Key));
    buf.push(Token::no_lexeme(TokenType::Assigner));

    lexeme.clear();
}

fn handle_separator(buf: &mut Vec<Token>, lexeme: &mut String) {
    buf.push(Token::new(lexeme, TokenType::Value));
    buf.push(Token::no_lexeme(TokenType::Separator));

    lexeme.clear();
}

pub fn lex(data: &str) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut lexeme = String::with_capacity(36);

    for ch in data.chars() {
        match ch {
            '{' => result.push(Token::no_lexeme(TokenType::OpeningBrace)),
            '}' => handle_closing_brace(&mut result, &mut lexeme),
            ':' => handle_assigner(&mut result, &mut lexeme),
            ',' => handle_separator(&mut result, &mut lexeme),
            '\n' | '\r' | '\t' | ' ' => (),
            _ => lexeme.push(ch),
        }
    }

    result
}

pub fn lex_from_file(path: &str) -> io::Result<Vec<Token>> {
    Ok(lex(&fs::read_to_string(path)?))
}
