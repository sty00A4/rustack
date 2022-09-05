#![allow(dead_code)]
#![allow(unused)]

use crate::lexer::TYPES::TYPE;

// GLOBALS
static DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

// FILE
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub text: String
}

// TOKEN
#[derive(Debug)]
pub enum TYPES {
    INT(isize), TYPE, SUB(Vec<Token>)
}
#[derive(Debug)]
pub struct Token {
    pub token: TYPES,
    pub start: usize,
    pub stop: usize
} impl Token {
    pub fn new(token: TYPES, start: usize, stop: usize) -> Self { Self { token, start, stop } }
}

// LEXER
#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    file: File,
    idx: usize
} impl Lexer {
    pub fn new(mut file: File) -> Self { Self { tokens: Vec::new(), file, idx: 0 } }
    pub fn advance(&mut self) { self.idx += 1 }
    pub fn char(&self) -> &str {
        if self.idx >= self.file.text.len() { return "" }
        &self.file.text[self.idx..self.idx+1]
    }
    pub fn range(&self, start: usize, stop: usize) -> &str { &self.file.text[start..stop] }
    pub fn next(&mut self) -> Result<(), String> {
        if DIGITS.contains(&self.char()) {
            let start = self.idx;
            while DIGITS.contains(&self.char()) { self.advance(); }
            let number = self.range(start, self.idx);
            self.tokens.push(Token::new(TYPES::INT(number.parse::<isize>().unwrap()), start, self.idx));
            return Ok(())
        }
        if self.char() == " " || self.char() == "\t" { self.advance(); return Ok(()) }
        return Err(String::from(format!("CHAR ERROR: '{}'", self.char())))
    }
    pub fn lex(&mut self) -> Result<(), String> {
        while self.char() != "" {
            let res = self.next();
            if res.is_err() { return Err(res.err().unwrap()) }
        }
        Ok(())
    }
}
pub fn lex(name: String, text: String) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(File{name, text});
    let res = lexer.lex();
    if res.is_err() { return Err(res.err().unwrap()) }
    return Ok(lexer.tokens)
}